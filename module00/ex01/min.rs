pub fn	min(a: i32, b: i32) -> i32
{
	if a < b
	{
		a
	}
	else
	{
		b
	}
}

fn	main()
{
	let mut a: i32;
	let mut b: i32;

	a = 2;
	b = 4;
	println!("The value of min between {} and {} is {}", a, b, min(a, b));
	a = 5;
	b = 5;
	println!("The value of min between {} and {} is {}", a, b, min(a, b));
	a = 6;
	b = 3;
	println!("The value of min between {} and {} is {}", a, b, min(a, b));
}
