# Boggle Solver #

## About ##

This is a program written in Rust that solves the game of Boggle. Boggle is a word game played on a grid of letters, where players try to find as many words as possible by connecting adjacent letters horizontally, vertically, or diagonally. [Wikipedia article](https://en.wikipedia.org/wiki/Boggle)

## Running ##

When run with no arguments a random board is generated for the Classic English version of boggle (4x4):

```bash
$ ./solve.sh 
Board:
 I  A  QU K 
 N  A  F  Y 
 D  W  N  A 
 E  T  A  L 
167 words found
== 8 letter words (1) ==
  ANTEDAWN
== 7 letter words (5) ==
  ANDANTE  FAINANT  LANATED  QUANTAL  QUANTED
...
```

A game type can be specified on the command line, as well as the minimum word length:

```bash
$ ./solve.sh --game big-original -m 5
Board:
 C A W T S
 P D A H A
 R D E J E
 W I O O N
 M D T S E
109 words found
== 9 letter words (1) ==
  STONEHEAD
== 7 letter words (8) ==
  HEADCAP  MIRDAHA  SNEATHE  SNOODED  SOOTIED  STOODED  TODIDAE  WHADDIE
...
```

A board layout can be specified on the command line, including dimensions:

```bash
$ ./solve.sh -y 3 -m 5 q w e r t y u i o p  a s d f g h j k l ''  z x c v b n m '' '' ''
Board:
 Q W E R T Y U I O P
 A S D F G H J K L █
 Z X C V B N M █ █ █
123 words found
== 6 letter words (1) ==
  SAWDER
== 5 letter words (12) ==
  DERTH  DEWAX  FERTH  POLKI  RESAW  REWAX  SAWED SAWER  SERGT  SWERD  TREWS  WEFTY
```

## Included word list ##

The included words.txt.gz file comes from [https://github.com/dwyl/english-words] which originally came from [https://www.infochimps.com/datasets/word-list-350000-simple-english-words-excel-readable].
