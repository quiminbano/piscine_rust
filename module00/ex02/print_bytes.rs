fn	print_bytes(s: &str)
{
	println!("Output:")
	for i in s.bytes()
	{
		println!("{}", i);
	}
}

fn	main()
{
	let mut buffer = String::new();
	let bytes_read: usize;

	println!("Input:")
	bytes_read = std::io::stdin()
				.read_line(&mut buffer)
				.expect("Error reading input!!");
	if bytes_read == 0
	{
		panic!("Mission aborted!! Thank you for using the program!");
	}
	print_bytes(&buffer);
}