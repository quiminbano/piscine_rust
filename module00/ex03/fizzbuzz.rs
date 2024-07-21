fn	fizzbuzz()
{
	for number in 1..101
	{
		match (number % 3 == 0, number % 5 == 0, number % 11 == 3, number % 11 == 5)
		{
			(true, false, _, _) => println!("fizz"),
			(false, true, _, _) => println!("buzz"),
			(true, true, _, _) => println!("fizzbuzz"),
			(false, false, true, false) => println!("FIZZ"),
			(false, false, false, true) => println!("BUZZ"),
			_ => println!("{}", number),
		}
	}
}

fn	main()
{
	fizzbuzz();
}