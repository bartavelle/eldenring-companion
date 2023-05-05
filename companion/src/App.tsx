import type { Component } from 'solid-js';

import styles from './App.module.css';
import Armament from './Armament'

const App: Component = () => {
  return (
    <div class={styles.App}>
      <header class={styles.header}>
          <Armament />
      </header>
    </div>
  );
};

export default App;
