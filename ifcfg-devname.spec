%bcond_without check
%global __cargo_skip_build 0

%global name ifcfg-devname

Name:           %{name}
Version:        0.1.0
Release:        1%{?dist}
Summary:        Udev helper utility that provides network interface naming using kernel cmdline and ifcfg configuration

License:        GPLv2

URL:            https://github.com/jamacku/%{name}
Source:         https://github.com/jamacku/%{name}/archive/%{version}.tar.gz#/%{name}-%{version}.tar.gz

ExclusiveArch:  %{rust_arches}

BuildRequires:  rust-toolset
BuildRequires:  rust-packaging
BuildRequires:  git

%description
Program ifcfg-devname reads ENV INTERFACE, which is expected to contain the name of the network interface. 
Then it looks for the MAC of such an interface. After that it looks at the kernel command line 
for key-value-pair ifname=NEW_NAME:MAC_ADDRESS with given MAC address. If a new name wasn't found and kernel 
cmdline it scans ifcfg configuration files in directory /etc/sysconfig/network-scripts/ and looks for configuration 
with HWADDR set to given hw address. If the program successfully finds such a configuration, it returns on standard 
output content of property DEVICE from matching ifcfg configuration. In all other cases it returns error code 1.

%prep
%autosetup -n %{name}-%{version_no_tilde} -p1
%cargo_prep

%generate_buildrequires
%cargo_generate_buildrequires -a

%build
%cargo_build -a

%check
%cargo_test -a

%install
%cargo_install -a

%files
# FIXME

%changelog
* Mon Oct 11 2021 Jan Macku <jamacku@redhat.com> - 0.1.0-1
- Init of ifcfg-devname package
