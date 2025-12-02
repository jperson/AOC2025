#!/usr/bin/env swipl
:- use_module(library(dcg/basics), [digits//1]).

:- initialization(main, main).

main(_) :-
    part1('./inputs/day1.txt', Part1),
    format("part1: ~w~n", [Part1]),
    part2('./inputs/day1.txt', Part2),
    format("part2: ~w~n", [Part2]).


part1(FileName, Count) :-
    phrase_from_file(rotations(Moves), FileName),
    solve_part1(Moves, Count).

part2(FileName, Count) :-
    phrase_from_file(rotations(Moves), FileName),
    solve_part2(Moves, Count).

rotations([]) --> [].
rotations([Inst|Rest]) --> 
    parse_rotation(Inst),
    rotations(Rest).

parse_rotation(rotate(Direction, Number)) -->
    [DirCode],
    { char_code(Direction, DirCode) },
    digits(NumCodes),
    { number_codes(Number, NumCodes) },
    [10].

solve_part1(Moves, Count) :-
    solve_part1(Moves, 50, 0, Count).

solve_part1([], _Current, Count, Count).

solve_part1([rotate('L', Amount)|Rest], Current, Acc, Count) :-
    NewCur is (Current - Amount) mod 100,
    (NewCur =:= 0 -> NewAcc is Acc + 1; NewAcc = Acc),
    solve_part1(Rest, NewCur, NewAcc, Count).

solve_part1([rotate('R', Amount)|Rest], Current, Acc, Count) :-
    NewCur is (Current + Amount) mod 100,
    (NewCur =:= 0 -> NewAcc is Acc + 1; NewAcc = Acc),
    solve_part1(Rest, NewCur, NewAcc, Count).

solve_part2(Moves, Count) :-
    solve_part2(Moves, 50, 0, Count).

solve_part2([], _Current, Count, Count).
solve_part2([rotate('L', Amount)|Rest], Current, Acc, Count) :-
    NewCur is (Current - Amount) mod 100,
    BaseCount is Acc + Amount // 100,
    ((NewCur > Current ; NewCur =:= 0), Current =\= 0 -> NewCount is BaseCount + 1; NewCount = BaseCount),
    solve_part2(Rest, NewCur, NewCount, Count).

solve_part2([rotate('R', Amount)|Rest], Current, Acc, Count) :-
    NewCur is (Current + Amount) mod 100,
    BaseCount is Acc + Amount // 100,
    ((NewCur < Current ; NewCur =:= 0), Current =\= 0 -> NewCount is BaseCount + 1; NewCount = BaseCount),
    solve_part2(Rest, NewCur, NewCount, Count).
