text
/* this is a
/* nested
block */ comment.
And should stay a comment
even with "strings"
or 42 0x18 0b01011   // blah
or u32 as i16 if impl */
text
/** this is a
/*! nested
block */ doc comment */
text

text
text // line comment
text /// line doc comment
text //! line doc comment
text

text #[allow(great_algorithms)]; text
text #[deny(silly_comments)] text

text 'single-quote string' text
text "double-quote string" text
text "string\nwith\x20escaped\"characters" text

text 42f32 text
text 42e+18 text
text 42.1415 text

text 42 text
text 0xf00b text
text 0o755 text
text 0b101010 text

text bool text char text uint text int text
text u8 text u16 text u32 text u64 text
text i8 text i16 text i32 text i64 text
text str text Self text

text true text false text

text break text continue text do text else text
text if text in text for text loop text
text match text return text while text

text as text crate text extern text mod text
text let text proc text ref text

text
extern crate foo;
text
use std::vec;
text
use std::{num, str};
text
use self::foo::{bar, baz};
text

text
pub enum MyEnum {
  One,
  Two
}
text

text
pub struct MyStruct {
  priv one: uint,
  two: MyEnum
}
text

text
type MyType = uint;
text

text
static MY_CONSTANT: u32 = 12345;
text

text
pub trait MyTrait {
  text
  fn create_something (param: &str, mut other_param: u32) -> Option<Self>;
  text
  fn do_whatever<T: Send+Pod+Whatever, U: Freeze> (param: &T, other_param: u32) -> Option<U>;
  text
  fn do_all_the_work (&mut self, param: &str, mut other_param: u32) -> bool;
  text
  fn do_even_more<T: Send+Whatever, U: Freeze> (&mut self, param: &T) -> U;
  text
}
text

text
impl MyTrait for MyStruct {
  text
  fn create_something (param: &str, mut other_param: u32) -> Option<Self> {
    text
    return Some(cake);
    text
  }
  text
  fn do_whatever<T: Send+Pod+Whatever, U: Freeze> (param: &T, other_param: u32) -> Option<U> {
    assert!(1 != 2);
    text
  }
  text
  fn do_all_the_work (&mut self, param: &str, mut other_param: u32) -> bool {
    info!("There's no cake");
    text
  }
  text
  fn do_even_more<T: Send+Whatever, U: Freeze> (&mut self, param: &T) -> U {
    text
  }
  text
}
text
