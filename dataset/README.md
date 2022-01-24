# 넘어진 글자 데셋

넘어진 글자 프로젝트에서 사용하는 데이터셋입니다.

## 프로젝트에 추가하기

```sh
$ git remote add dataset git@github.com:numgle/dataset.git
$ git subtree add --prefix dataset dataset main
```

`/dataset`에 subtree가 만들어집니다.

`/dataset/src/data.json`를 불러와 구현합니다.

## 가져오고 수정사항 반영하기

```sh
$ git subtree pull --prefix dataset dataset main

$ git subtree push --prefix dataset dataset main
```
