%bcond_without check
%global __cargo_skip_build 0

%global crate ifcfg-devname

Name:           rust-%{crate}
Version:        0.1.0
Release:        1%{?dist}
Summary:        # FIXME

# Upstream license specification: None
License:        # FIXME

URL:            https://crates.io/crates/ifcfg-devname
Source:         %{crates_source}

ExclusiveArch:  %{rust_arches}

BuildRequires:  rust-packaging

%global _description %{expand:
%{summary}.}

%description %{_description}

%package     -n %{crate}
Summary:        %{summary}

%description -n %{crate} %{_description}

%files       -n %{crate}
%doc README.md
%{_bindir}/ifcfg-devname

%prep
%autosetup -n %{crate}-%{version_no_tilde} -p1
%cargo_prep

%generate_buildrequires
%cargo_generate_buildrequires -a

%build
%cargo_build -a

%install
%cargo_install -a

%if %{with check}
%check
%cargo_test -a
%endif

%changelog
