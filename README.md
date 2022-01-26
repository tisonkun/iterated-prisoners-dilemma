# Iterated Prisoner's Dilemma Emulator

## Name

This repository provides an emulator
for [iterated prisoner's dilemma](https://en.wikipedia.org/wiki/Prisoner%27s_dilemma#The_iterated_prisoner's_dilemma).

## Description

You can run the program by the follow command:

```shell
cargo run
```

Results look like:

```text
...
Strategy            Score
AlwaysBetray        60044
AlwaysCooperate     67581
HalfToHalf          58721
TitForTat           76298
AlwaysRevenge       80016
```

You're encouraged to modify the source code especially to add new strategy.

To add a new strategy, you should create a new file under the [src/strategy](src/strategy) folder, implement
the `Strategy` Trait, and register it in the [main.rs](src/main.rs) file.

* For a simple strategy example that doesn't care about opponents history, see
  the [`AlwaysCooperate`](src/strategy/always_cooperate.rs) strategy.
* For a simple strategy example that thinks of opponents history, see the [`TitForTat`](src/strategy/tit_for_tat.rs)
  strategy.

## License

The bundle itself is licensed under [the MIT license](LICENSE).

## Reference

Thanks to [*The Evolution of Cooperation*](https://en.wikipedia.org/wiki/The_Evolution_of_Cooperation) for theoretical background.
