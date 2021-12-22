true.

X = X.

plus(1,1,2).

food(burger).
food(sandwich).
food(pizza).
lunch(sandwich).
lunch(pizza).
dinner(pizza).


meal(X) :- food(X).
always(X) :- lunch(X), dinner(X).

nat(0).
nat(s(X)) :- nat(X).

append([], L, L).
append([H|T], L, [H|R]) :-
    append(T, L, R).
