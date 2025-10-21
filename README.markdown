# DOLLSCRIPT :ribbon: :dolls:

![Dollscript CI](https://github.com/alyxshang/dollscript/actions/workflows/dollscript.yml/badge.svg)

***A light and fast interpreter for a language for dramatic dollies. :ribbon: :dolls:***

## ABOUT :books:

This repository contains the source code for an interpreter for a
programming language called ***Dollscript***. ***Dollscript*** is a
language of my own design and conception. It was inspired by the 
[*BratzDolls*](https://en.wikipedia.org/wiki/Bratz) and to a lesser 
degree by the culture around [*Yami Kawaii*](https://aesthetics.fandom.com/wiki/Yami_Kawaii).

## LANGUAGE SAMPLE :gear:

```Text
inspo "std" as std;

slayy bag Person{
    name: tea,
    age: cash,
}

slayy move makePerson(name: tea, age: cash): Person{
    Person{ name, age }
}

slayy move greetPerson(person: *Person): sleep {
    std::print("Hello from Dollscript, ${person.name}!");
}

move main(): sleep {
    wisdom purrson: Person = makePerson("Alyx", 2025);
    greetPerson(*purrson);
}
```

## TO DO :warning:

- [x] Lexer.
- [ ] Parser.
- [ ] Intermediate-representation-generator.
- [ ] Instructionset-generator.
- [ ] Instructionset-executor.
- [ ] Linter.
- [ ] Modulesystem.

## NOTE :scroll:

- *Dollscript :ribbon: :dolls:* by *Alyx Shang :black_heart:*.
- Licensed under the [FSL v1](https://github.com/alyxshang/fair-software-license).
