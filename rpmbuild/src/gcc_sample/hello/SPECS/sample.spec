Name:    hello
Version: 1.0.0
Release: 1
Summary: hello test command

License: Unlicense
Source0: hello

%define INSTALLDIR /usr/local/bin

%description
hello test command.

%install
mkdir -p %{buildroot}%{INSTALLDIR}
install -p -m 755 %{SOURCE0} %{buildroot}%{INSTALLDIR}

%files
%attr(0755, root, root) %{INSTALLDIR}/*

%clean
rm -rf %{buildroot}%{INSTALLDIR}
