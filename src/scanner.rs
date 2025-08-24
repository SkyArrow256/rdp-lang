#[derive(Debug)]
pub struct Scanner<'a> {
	tokens: Vec<Token<'a>>,
	index: usize,
}

impl<'a> From<&'a str> for Scanner<'a> {
	fn from(text: &'a str) -> Self {
		let tokens = split(text);
		Self { tokens, index: 0 }
	}
}

impl<'a> Scanner<'a> {
	/// 今のトークンを取得します。
	pub fn peek(&'a self) -> Option<&'a Token<'a>> {
		self.tokens.get(self.index)
	}
	/// 今のトークンを取り出して次に進めます。
	pub fn take(&'a mut self) -> Option<&'a Token<'a>> {
		self.index += 1;
		self.tokens.get(self.index - 1)
	}
	/// 今のトークンが指定された型と等しいか確認します。
	pub fn is_match(&self, t: Token) -> bool {
		if let Some(token) = self.peek() {
			t == *token
		} else {
			false
		}
	}
}

fn split<'a>(text: &'a str) -> Vec<Token<'a>> {
	let mut tokens = Vec::new();
	let mut index = 0;
	while let Some(x) = text.chars().nth(index) {
		match x {
			//文字列リテラルの時
			'"' => {
				println!("\tString");
				let to = index + text[index+1..].find('"').unwrap() + 1;
				tokens.push(tokenize(&text[index..=to]));
				println!("{}", &text[index..=to]);
				index = to + 1;
			}
			//数字の時
			c @ _ if c.is_ascii_digit() => {
				println!("\tNumber");
				let mut to = 0;
				while let Some(i) = text.chars().nth(index + to) && i.is_ascii_digit() {
					to += 1;
				}
				tokens.push(tokenize(&text[index..index+to]));
				index += to;
			}
			//文字列(命令など)の時
			c @ _ if c.is_ascii_alphabetic() => {
				println!("\tIdent");
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
				println!("\tSpace");
				index += 1;
			}
			//それ以外はそのままトークン化
			_ => {
				tokens.push(tokenize(&text[index..index+1]));
				index += 1;
			}
		}
		println!("index: {index}\ntokens: {tokens:?}");
	}
	tokens
}

fn tokenize<'a>(word: &'a str) -> Token<'a> {
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
				Token::String(str.trim_matches('"'))
			} else {
				Token::Ident(str)
			}
		}
	}
}

#[derive(Debug, PartialEq)]
pub enum Token<'a> {
	FuncDef,
	ParenOpen,
	ParenClose,
	Begin,
	End,
	Semicolon,
	String(&'a str),
	Ident(&'a str),
}