Name:    hello
Version: 1.0.0
Release: 1
Summary: hello test command
License: Unlicense

%define INSTALLDIR /usr/local/bin

%description
hello test command.

%install
mkdir -p %{buildroot}%{INSTALLDIR}
install -p -m 755 $SRC/hello %{buildroot}%{INSTALLDIR}

%files
%attr(0755, root, root) %{INSTALLDIR}/*

%clean
rm -rf %{buildroot}%{INSTALLDIR}

# Build
# export SRC=$(pwd) && rpmbuild -bb hello.spec

# Install
# rpm -ivh /root/rpmbuild/RPMS/x86_64/hello-1.0.0-1.x86_64.rpm
