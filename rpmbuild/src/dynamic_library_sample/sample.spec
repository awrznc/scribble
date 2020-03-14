Name:    sample
Version: 1.0.0
Release: 1
Summary: sample test command
License: Unlicense

%define INSTALLDIR /usr/local/sample

%description
sample test command.

%install
mkdir -p %{buildroot}%{INSTALLDIR}
mkdir -p %{buildroot}%{INSTALLDIR}/bin
mkdir -p %{buildroot}%{INSTALLDIR}/lib
install -p -m 755 $SRC/sample %{buildroot}%{INSTALLDIR}/bin
install -p -m 755 $SRC/libsample.so %{buildroot}%{INSTALLDIR}/lib
echo "/usr/local/sample/lib" > /etc/ld.so.conf.d/sample.conf
ldconfig

%files
%attr(0755, root, root) %{INSTALLDIR}/*

%clean
rm -rf %{buildroot}%{INSTALLDIR}
