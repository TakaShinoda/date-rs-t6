# date-rs-t6

## Install
```
npm i date-rs-t6
```

## Usage
```vue
<script setup lang="ts">
import { local_now } from 'date-rs-t6'

const date = local_now()
</script>

<template>
  <!-- Date in local time yyyy/mm/dd  -->
  <!-- now: 2024/10/25 -->
  <p>now: {{ date }}</p>
</template>

```