use srack_lang::interpreter::run;

#[test]
fn hello_world() {
    let source = "\
seed 42
push World
push blahblahblah
push Hello
call println
poop
call println";


    let mut buffer: Vec<u8> = Vec::new();
    run(source.to_string(), false, &mut buffer);
    let actual = String::from_utf8(buffer).unwrap();
    assert_eq!(actual, "Hello\nWorld\n")
}

#[test]
fn math() {
    let source = "\
seed 42
push 1
push blahblahblah
push 2
push blahblahblah
call +
call println";


    let mut buffer: Vec<u8> = Vec::new();
    run(source.to_string(), false, &mut buffer);
    let actual = String::from_utf8(buffer).unwrap();
    assert_eq!(actual, "3\n")
}

