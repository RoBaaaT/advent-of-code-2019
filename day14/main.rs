use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

#[derive(Debug)]
struct Reactant {
    chemical: usize,
    amount: usize
}

#[derive(Debug)]
struct Reaction {
    output: Reactant,
    inputs: Vec<Reactant>
}

fn parse_reactant(string: &str, chemicals: &mut Vec<String>) -> Reactant {
    let amount_chemical: Vec<&str> = string.trim().split(" ").collect();
    assert_eq!(amount_chemical.len(), 2);
    let chemical = match chemicals.iter().position(|x| x == amount_chemical[1]) {
        Some(pos) => pos,
        None => { chemicals.push(amount_chemical[1].to_string()); chemicals.len() - 1 }
    };
    let amount = amount_chemical[0].parse().unwrap();
    Reactant { chemical: chemical, amount: amount }
}

// assumes that the reaction for output i is reactions[i]
fn required_chemical_amount(reactions: &Vec<Reaction>, leftover_amounts: &mut Vec<usize>, input_chemical: usize,
        output_chemical: usize, output_amount: usize) -> usize {
    assert_eq!(reactions[output_chemical].output.chemical, output_chemical);
    let mut required_amount = 0;
    let required_output_amount = if output_amount > leftover_amounts[output_chemical] { output_amount - leftover_amounts[output_chemical] } else { 0 };
    let reaction_count = (required_output_amount + reactions[output_chemical].output.amount - 1) / reactions[output_chemical].output.amount;
    leftover_amounts[output_chemical] = leftover_amounts[output_chemical] + reaction_count * reactions[output_chemical].output.amount - output_amount;
    for input in &reactions[output_chemical].inputs {
        if input.chemical == input_chemical {
            required_amount += input.amount * reaction_count;
        } else {
            required_amount += required_chemical_amount(reactions, leftover_amounts, input_chemical,
                input.chemical, input.amount * reaction_count);
        }
    }
    required_amount
}

fn main() {
    let input_file = File::open("input.txt").unwrap();
    let input = BufReader::new(&input_file);

    let mut chemicals: Vec<String> = vec!["ORE".to_string(), "FUEL".to_string()];
    let mut reactions = Vec::new();

    reactions.push(Reaction { output: Reactant { chemical: 0, amount: 0 }, inputs: vec![] });
    for line in input.lines() {
        let reaction = line.unwrap();
        let inputs_outputs: Vec<&str> = reaction.split("=>").collect();
        assert_eq!(inputs_outputs.len(), 2);
        let output = parse_reactant(inputs_outputs[1], &mut chemicals);
        let mut inputs = Vec::new();
        for input in inputs_outputs[0].split(",") {
            inputs.push(parse_reactant(input, &mut chemicals));
        }
        reactions.push(Reaction { output: output, inputs: inputs });
    }

    reactions.sort_by(|a, b| a.output.chemical.cmp(&b.output.chemical));

    // part 1
    let mut leftover_amounts = vec![0; reactions.len()];
    let required_ore = required_chemical_amount(&reactions, &mut leftover_amounts, 0, 1, 1);
    println!("Part 1: {}", required_ore);

    // part 2
    let mut bottom = 1;
    let mut top = 1000000000000;
    while bottom < top {
        let middle = (bottom + top) / 2;
        let mut leftover_amounts = vec![0; reactions.len()];
        let required_ore = required_chemical_amount(&reactions, &mut leftover_amounts, 0, 1, middle);

        if required_ore == 1000000000000 {
            break;
        } else {
            if required_ore > 1000000000000 {
                top = middle - 1;
            } else {
                bottom = middle + 1;
            }
        }
    }
    println!("Part 2: {}", (bottom + top) / 2);
}