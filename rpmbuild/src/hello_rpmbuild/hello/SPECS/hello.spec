Name:    hello
Version: 1
Release: 1%{?dist}
Summary: hello test command

License: No License No Life
Source0: hello

%description
hello test command!!

%install
mkdir -p %{buildroot}/%{_bindir}
install -p -m 755 %{SOURCE0} %{buildroot}/%{_bindir}

%files
%{_bindir}/hello
