yenru0 code storage
===

## 기본 사항

폴더명 `{identifier}_{language}`

`identifier` 식별자로는 소스 문제로, 의 종류는 다음과 같음.

* `zeta`: [BOJ](https://www.acmicpc.net/)

`language`

작성언어 | 키워드 | 확장자
:---:|:---:|:---:
 C | C | .c
 C++ | cpp | .cpp
 Python 또는 pypy | python | .py
 Kotlin | kotlin | .kt
 Lua | lua | .lua

### completed or incomplete


해결한 문제는 `folder/completed`로 이동.

해결되지 않은 문제는 `/`에 잔류.

## code editing 

## script

실행을 위한 파이썬 스크립트가 있다.

* `run`

```sh

# 테스트 케이스 생성 with count=5
run init 5

# workspace로 불러오는 file
run load 1000 --to py # load 1000.py from python source space
# if there loaded file
run load 1000.py # language inference by extension
run load 1000.py --from zeta
run load 1000.rs --from zeta --force # already in src space force overwrite


run export --from c --completed

run time rust.it

```

## .config.yml

현재 source space에 있는 언어, 파일, 식별자, 파일을 기록해놓음.

run load 시 다음 파일에 그런 형식을 작성함.

export 시에는 모든 config가 non-null이어야 함.







## 시간 측정

### python

```
python -m profile files.py < stdin.txt
```
