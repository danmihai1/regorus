// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

#![no_main]
extern crate regorus;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
	let source = regorus::lexer::Source::new("source.rego".to_owned(), s.to_owned());
	let mut lexer = regorus::lexer::Lexer::new(&source);
	loop {
	    match lexer.next_token() {
		Ok(t) => {
		    if t.0 == regorus::lexer::TokenKind::Eof {
			break;
		    }			
		}
		_ => break
	    }
	}
    }
});
