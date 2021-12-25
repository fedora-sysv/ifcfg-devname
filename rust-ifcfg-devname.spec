%bcond_without check

%global crate ifcfg-devname

Name:           rust-%{crate}
Version:        0.1.3
Release:        %autorelease
Summary:        Udev helper utility that provides network interface naming using kernel cmdline and ifcfg configuration

License:        GPLv3
URL:            https://crates.io/crates/ifcfg-devname
Source:         %{crates_source}

ExclusiveArch:  %{rust_arches}

BuildRequires:  rust-packaging

%global _description %{expand:
Program ifcfg-devname reads ENV INTERFACE, which is expected to contain the name of the network interface. 
Then it looks for the MAC of such an interface. After that it looks at the kernel command line 
for key-value-pair ifname=NEW_NAME:MAC_ADDRESS with given MAC address. If a new name wasn't found and kernel 
cmdline it scans ifcfg configuration files in directory /etc/sysconfig/network-scripts/ and looks for configuration 
with HWADDR set to given hw address. If the program successfully finds such a configuration, it returns on standard 
output content of property DEVICE from matching ifcfg configuration. In all other cases it returns error code 1.}

%description    %{_description}

%package     -n %{crate}
Summary:        %{summary}

%description -n %{crate} %{_description}

%files       -n %{crate}
%license LICENSE
%doc README.md
%{_bindir}/ifcfg-devname

%prep
%autosetup   -n %{crate}-%{version_no_tilde} -p1
%cargo_prep

%generate_buildrequires
%cargo_generate_buildrequires

%build
%cargo_build

%install
%cargo_install

%if %{with check}
%check
%cargo_test
%endif

%changelog
%autochangelog
