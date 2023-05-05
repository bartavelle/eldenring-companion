import type { Component } from 'solid-js';

import styles from './App.module.css';
import Armament from './Armament'
import Armor from './Armor'

const App: Component = () => {
  return (
    <div class={styles.App}>
      <header class={styles.header}>
          <Armor />
      </header>
    </div>
  );
};

export default App;
