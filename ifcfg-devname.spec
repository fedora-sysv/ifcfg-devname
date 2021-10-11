%bcond_without check
%global __cargo_skip_build 0

%global name ifcfg-devname

Name:           %{name}
Version:        0.1.0
Release:        1%{?dist}
Summary:        # FIXME

License:        GPLv2

URL:            https://github.com/jamacku/%{name}
Source:         https://github.com/jamacku/%{name}/archive/%{version}.tar.gz#/%{name}-%{version}.tar.gz

ExclusiveArch:  %{rust_arches}

BuildRequires:  rust-toolset
BuildRequires:  rust-packaging
BuildRequires:  git

%description
# FIXME

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
