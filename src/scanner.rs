use std::mem;

#[derive(Debug)]
pub struct Scanner {
	tokens: Vec<Token>,
	index: usize,
}

impl From<&str> for Scanner {
	fn from(text: &str) -> Self {
		let tokens = split(text);
		Self { tokens, index: 0 }
	}
}

impl Scanner {
	/// 今のトークンを複製して取得します。
	pub fn peek(&self) -> Option<Token> {
		self.tokens.get(self.index).cloned()
	}
	/// 今のトークンを取り出して次に進めます。
	pub fn take(&mut self) -> Option<Token> {
		self.index += 1;
		self.tokens.get(self.index - 1).cloned()
	}
	/// 今のトークンが指定された型と等しいか確認します。
	pub fn is_match(&self, t: Token) -> bool {
		if let Some(token) = self.peek() {
			mem::discriminant(&t) == mem::discriminant(&token)
		} else {
			false
		}
	}
}

fn split(text: &str) -> Vec<Token> {
	let mut tokens = Vec::new();
	let mut index = 0;
	while let Some(x) = text.chars().nth(index) {
		match x {
			//文字列リテラルの時
			'"' => {
				let to = index + text[index+1..].find('"').unwrap() + 1;
				tokens.push(tokenize(&text[index..=to]));
				index = to + 1;
			}
			//数字の時
			c @ _ if c.is_ascii_digit() => {
				let mut to = 0;
				while let Some(i) = text.chars().nth(index + to) && i.is_ascii_digit() {
					to += 1;
				}
				tokens.push(tokenize(&text[index..index+to]));
				index += to;
			}
			//文字列(命令など)の時
			c @ _ if c.is_ascii_alphabetic() => {
				let mut to = 0;
				while let Some(i) = text.chars().nth(index + to) && i.is_ascii_alphabetic() {
					println!("{i}");
					to += 1;
				}
				tokens.push(tokenize(&text[index..index+to]));
				index += to;
			}
			//空白はスキップ
			c @ _ if c.is_whitespace() => {
				index += 1;
			}
			//それ以外はそのままトークン化
			_ => {
				tokens.push(tokenize(&text[index..index+1]));
				index += 1;
			}
		}
	}
	tokens
}

fn tokenize(word: &str) -> Token {
	println!("\t\t{word}");
	match word {
		"fn" => Token::FuncDef,
		"(" => Token::ParenOpen,
		")" => Token::ParenClose,
		"{" => Token::Begin,
		"}" => Token::End,
		";" => Token::Semicolon,
		str @ _ => {
			if str.chars().nth(0) == Some('"') && str.chars().nth_back(0) == Some('"') {
				Token::String(str.trim_matches('"').to_string())
			} else {
				Token::Ident(str.to_string())
			}
		}
	}
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
	FuncDef,
	ParenOpen,
	ParenClose,
	Begin,
	End,
	Semicolon,
	String(String),
	Ident(String),
}