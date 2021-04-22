import { AppBar, Container, Toolbar, Typography } from '@material-ui/core';
import React from 'react';
import TodoList from './features/todos/TodoList';

const App = () => {

  return (
    <div>
      <AppBar position="static">
        <Toolbar>
          <Typography variant="h3">Todos</Typography>
        </Toolbar>
      </AppBar>
      <Container maxWidth="sm">
        <TodoList />
      </Container>
    </div>
  )
}

export default App;
