use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    pub interface: String,
    pub port: String,
    pub comment_remover: CommentRemoverConfig,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CommentRemoverConfig {
    pub html: bool,
    pub js: bool,
    pub css: bool,
}

impl Config {
    pub fn overwrite(&mut self, new_conf: Config) {
        self.interface = new_conf.interface;
        self.port = new_conf.port;
        self.comment_remover.overwrite(new_conf.comment_remover);
    }
}

impl CommentRemoverConfig {
    pub fn overwrite(&mut self, new_rem: CommentRemoverConfig) {
        self.html = new_rem.html;
        self.js = new_rem.js;
        self.css = new_rem.css;
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            interface: "127.0.0.1".to_owned(),
            port: "8000".to_owned(),
            comment_remover: Default::default(),
        }
    }
}

impl Default for CommentRemoverConfig {
    fn default() -> Self {
        Self {
            html: false,
            js: false,
            css: false,
        }
    }
}
