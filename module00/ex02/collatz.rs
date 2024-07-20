fn	collatz(start: u32)
{
	let mut copy: u32;

	copy = start;
	println!("Output:\n{}", copy);
	while copy != 1
	{
		if copy % 2 == 0
		{
			copy /= 2;
		}
		else
		{
			copy = (3 * copy) + 1
		}
		println!("{}", copy)
	}
}

fn	main()
{
	let mut buffer = String::new();
	let bytes_read: usize;
	let start: u32;

	println!("Input");
	bytes_read = std::io::stdin()
				.read_line(&mut buffer)
				.expect("An error ocurred reading the file!");
	if bytes_read == 0
	{
		panic!("Aborted mission!, Thank you for using the program!");
	}
	start = buffer.trim()
			.parse()
			.expect("The program couldn't convert the input into a number!");
	collatz(start);
}
