/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 *
 * Copyright 2017 - Dario Ostuni <dario.ostuni@gmail.com>
 *
 */

extern crate btpc;

use std::env;

fn main()
{
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() == 0
    {
        panic!("Usage: is_prime <number [number[...]]>");
    }
    for arg in &args
    {
        let n = arg.parse::<usize>().unwrap();
        print!("{} is ", n);
        if !btpc::is_prime(n)
        {
            print!("not ");
        }
        println!("prime!");
    }
}
