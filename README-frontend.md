nodejs framework 없이는 hot reload가 안되는것 같다.
당분간 nodejs framework을 쓰면서 svelte, vue, react를 공부한다.

## Project setup

svelte, typescipt

```
yarn create vite
yarn add -D @tauri-apps/cli
yarn tauri init
```

## Run

```
yarn tauri dev

```

## yarn install on Ubuntu

```
sudo apt install npm
sudo npm install -g yarn
```

## ui는 vue+quasar 로 결정

### svelte

간명하고 쉽게 배울수 있는데 이쁜 ui가 별로 없다.
sveltekit, sveltstrap, unocss, smui, carbon, daisyui 등을 시도해 보았다.
daisyui가 가장 그럴듯한데 이것도 css 속성 같은게 많이 나온다.

### react

html 을 함수에서 return 한다. code 읽기 쉽지 않다.

### vuetify

아직 vue3 미지원

## Link

- How to return the result of an asynchronous function in JavaScript

  https://flaviocopes.com/how-to-return-result-asynchronous-function/
