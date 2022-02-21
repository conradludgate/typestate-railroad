use railroad::*;
mod css;

fn main() {
    let mut dia = Diagram::new(config_builder());
    dia.add_element(
        railroad::svg::Element::new("style")
            .set("type", "text/css")
            .raw_text(css::CSS),
    );
    println!("{}", dia);
}

fn func(ident: &str) -> Box<dyn RailroadNode> {
    Box::new(NonTerminal::new(ident.to_string()))
}
fn from_state(ident: &str, inner: impl RailroadNode + 'static) -> Box<dyn RailroadNode> {
    Box::new(LabeledBox::new(inner, Terminal::new(ident.to_string())))
}

fn config_builder() -> Box<dyn RailroadNode> {
    Box::new(VerticalGrid::new(vec![
        server::config_builder(),
        client::config_builder(),
    ]))
}

mod common {
    use super::*;

    pub fn wants_cipher_suites() -> Box<dyn RailroadNode> {
        from_state(
            "ConfigBuilder<_, WantsCipherSuites>",
            Choice::new(vec![
                func("with_safe_defaults"),
                Box::new(Sequence::new(vec![
                    specify_cipher_suites(),
                    wants_kx_groups(),
                    wants_versions(),
                ])),
            ]),
        )
    }

    fn specify_cipher_suites() -> Box<dyn RailroadNode> {
        Box::new(Choice::new(vec![
            func("with_cipher_suites"),
            func("with_safe_default_cipher_suites"),
        ]))
    }

    fn wants_kx_groups() -> Box<dyn RailroadNode> {
        from_state(
            "ConfigBuilder<_, WantsKxGroups>",
            Choice::new(vec![
                func("with_kx_groups"),
                func("with_safe_default_kx_groups"),
            ]),
        )
    }

    fn wants_versions() -> Box<dyn RailroadNode> {
        from_state(
            "ConfigBuilder<_, WantsVersions>",
            Choice::new(vec![
                func("with_protocol_versions"),
                func("with_safe_default_protocol_versions"),
            ]),
        )
    }
}

mod server {
    use super::*;

    pub fn config_builder() -> Box<dyn RailroadNode> {
        Box::new(Sequence::new(vec![
            Box::new(SimpleStart),
            common::wants_cipher_suites(),
            server::wants_verifier(),
            server::wants_server_cert(),
            Box::new(SimpleEnd),
        ]))
    }

    pub fn wants_verifier() -> Box<dyn RailroadNode> {
        from_state(
            "ConfigBuilder<ServerConfig, WantsVerifier>",
            Choice::new(vec![
                func("with_client_cert_verifier"),
                func("with_no_client_auth"),
            ]),
        )
    }

    pub fn wants_server_cert() -> Box<dyn RailroadNode> {
        from_state(
            "ConfigBuilder<ServerConfig, WantsServerCert>",
            Choice::new(vec![
                func("with_single_cert"),
                func("with_single_cert_with_ocsp_and_sct"),
                func("with_cert_resolver"),
            ]),
        )
    }
}

mod client {
    use super::*;

    pub fn config_builder() -> Box<dyn RailroadNode> {
        Box::new(Sequence::new(vec![
            Box::new(SimpleStart),
            common::wants_cipher_suites(),
            client::wants_verifier(),
            Box::new(SimpleEnd),
        ]))
    }

    pub fn wants_verifier() -> Box<dyn RailroadNode> {
        from_state(
            "ConfigBuilder<ClientConfig, WantsVerifier>",
            Choice::new(vec![
                Box::new(Sequence::new(vec![
                    func("with_root_certificates"),
                    wants_transparency_policy_or_client_cert(),
                ])),
                Box::new(Sequence::new(vec![
                    func("with_custom_certificate_verifier"),
                    wants_client_cert(),
                ])),
            ]),
        )
    }

    pub fn wants_transparency_policy_or_client_cert() -> Box<dyn RailroadNode> {
        from_state(
            "ConfigBuilder<ClientConfig, WantsTransparencyPolicyOrClientCert>",
            Choice::new(vec![
                Box::new(Sequence::new(vec![
                    func("with_certificate_transparency_logs"),
                    wants_client_cert(),
                ])),
                func("with_single_cert"),
                func("with_no_client_auth"),
                func("with_client_cert_resolver"),
            ]),
        )
    }

    pub fn wants_client_cert() -> Box<dyn RailroadNode> {
        from_state(
            "ConfigBuilder<ClientConfig, WantsClientCert>",
            Choice::new(vec![
                func("with_single_cert"),
                func("with_no_client_auth"),
                func("with_client_cert_resolver"),
            ]),
        )
    }
}
