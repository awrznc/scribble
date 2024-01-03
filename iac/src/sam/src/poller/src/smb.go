package main

import (
	"crypto/md5"
	"fmt"
	"io"
	iofs "io/fs"
	"io/ioutil"
	"net"
	"os"
	"regexp"

	"github.com/hirochachacha/go-smb2"
)

type Samba struct {
	ip       string
	port     string
	share    string
	prefix   string
	user     string
	password string
}

func (samba *Samba) commonProcess(params []string, proc func(*smb2.Share, *Samba, []string) (map[string]string, error)) (map[string]string, error) {

	// sambaへ接続
	endpoint := fmt.Sprintf("%v:%v", samba.ip, samba.port)
	connection, err := net.Dial("tcp", endpoint)
	if err != nil {
		return map[string]string{}, err
	}
	defer connection.Close()

	// sambaで認証
	dialer := &smb2.Dialer{
		Initiator: &smb2.NTLMInitiator{
			User:     samba.user,
			Password: samba.password,
		},
	}
	session, err := dialer.Dial(connection)
	if err != nil {
		return map[string]string{}, err
	}
	defer session.Logoff()

	// sambaをマウント
	fs, err := session.Mount(fmt.Sprintf(`\\%s\%s`, endpoint, samba.share))
	if err != nil {
		return map[string]string{}, err
	}
	defer fs.Umount()

	return proc(fs, samba, params)
}

type SmbPathElements struct {
	endpoint string
	share    string
	target   string
}

func parseSmbPath(path string) (SmbPathElements, error) {
	regexpString := regexp.MustCompile(`^\\\\(.+?)\\(.+?)\\(.+)$`)
	result := regexpString.FindAllStringSubmatch(path, -1)

	if len(result) != 1 || len(result[0]) != 4 {
		return SmbPathElements{}, fmt.Errorf("parse error ( path: %s )", path)
	}

	return SmbPathElements{
		endpoint: result[0][1],
		share:    result[0][2],
		target:   result[0][3],
	}, nil
}

func ccheck(data io.Reader) (string, error) {
	hash := md5.New()
	if _, err := io.Copy(hash, data); err != nil {
		return "", err
	}

	return fmt.Sprintf("%x", hash.Sum(nil)), nil
}

func lsFileOnlyRecursive(fs *smb2.Share, samba *Samba, ignorePaths []string) (map[string]string, error) {

	hashmap := map[string]string{}

	err := iofs.WalkDir(fs.DirFS(`.`), `./`+samba.prefix, func(path string, entry iofs.DirEntry, err error) error {
		if err != nil {
			return err
		}
		if !entry.IsDir() {
			read, err := fs.Open(path)
			if err != nil {
				panic(err)
			}
			var key string
			if len(samba.prefix) == 0 {
				key = path
			} else {
				key = path[len(samba.prefix)+1:]
			}
			hashmap[key], err = ccheck(read)
			if err != nil {
				panic(err)
			}
		}

		return nil
	})

	if err != nil {
		return hashmap, nil
	}

	return hashmap, nil
}

func cp(fs *smb2.Share, _ *Samba, params []string) (map[string]string, error) {

	paramsLength := len(params)
	if paramsLength != 2 {
		return map[string]string{}, fmt.Errorf("cp: not enough parameters ( length: %d )", paramsLength)
	}

	from := params[0]
	to := params[1]
	smbPath, err := parseSmbPath(from)
	if err != nil {
		return map[string]string{}, err
	}

	read, err := fs.Open(smbPath.target)
	if err != nil {
		return map[string]string{}, err
	}

	write, err := os.Create(to)
	if err != nil {
		return map[string]string{}, err
	}

	_, err = io.Copy(write, read)
	if err != nil {
		return map[string]string{}, err
	}
	return map[string]string{}, nil
}

func (samba *Samba) GetData(sufix string) ([]byte, error) {
	// sambaへ接続
	endpoint := fmt.Sprintf("%v:%v", samba.ip, samba.port)
	connection, err := net.Dial("tcp", endpoint)
	if err != nil {
		return nil, err
	}
	defer connection.Close()

	// sambaで認証
	dialer := &smb2.Dialer{
		Initiator: &smb2.NTLMInitiator{
			User:     samba.user,
			Password: samba.password,
		},
	}
	session, err := dialer.Dial(connection)
	if err != nil {
		return nil, err
	}
	defer session.Logoff()

	// sambaをマウント
	fs, err := session.Mount(fmt.Sprintf(`\\%s\%s`, endpoint, samba.share))
	if err != nil {
		return nil, err
	}
	defer fs.Umount()

	prefix := ``
	if samba.prefix != `` {
		prefix = samba.prefix + `/`
	}
	read, err := fs.Open(prefix + sufix)
	if err != nil {
		return nil, err
	}

	result, err := ioutil.ReadAll(read)

	if err != nil {
		return nil, err
	}
	return result, nil
}

func (samba *Samba) Ls() (map[string]string, error) {
	return samba.commonProcess([]string{}, lsFileOnlyRecursive)
}

func (samba *Samba) Cp(from string, to string) (map[string]string, error) {
	return samba.commonProcess([]string{from, to}, cp)
}
