```js
<script setup>
import { ref, onMounted } from "vue";

const ws = ref(null);
const messages = ref([]);

// Get dynamic port based on current URL
const wsPort = window.location.port ? parseInt(window.location.port) + 1 : 8081;
const wsUrl = `ws://${window.location.hostname}:${wsPort}/updates`;

onMounted(() => {
  ws.value = new WebSocket(wsUrl);

  ws.value.onmessage = (event) => {
    messages.value.push(event.data);
  };

  ws.value.onopen = () => {
    console.log("WebSocket connected!", wsUrl);
  };

  ws.value.onerror = (error) => {
    console.error("WebSocket error:", error);
  };

  ws.value.onclose = () => {
    console.log("WebSocket disconnected.");
  };
});
</script>

<template>
  <div>
    <h2>Live Updates</h2>
    <ul>
      <li v-for="msg in messages" :key="msg">{{ msg }}</li>
    </ul>
  </div>
</template>
```

```js
<template>
  <input v-model="message" placeholder="Type a message" />
  <button @click="sendMessage">Send</button>
</template>

<script setup>
const message = ref("");

const sendMessage = () => {
  if (ws.value && ws.value.readyState === WebSocket.OPEN) {
    ws.value.send(message.value);
  }
};
</script>
```
