import { Box, Stack, Typography } from "@mui/material";
import { ThemeProvider, createTheme } from "@mui/material/styles";
import "modern-css-reset";
import { FC, useEffect, useState } from "react";
import SideNav from "./components/SideNav";
import TodoForm from "./components/TodoForm";
import TodoList from "./components/TodoList";
import { addLabelItem, deleteLabelItem, getLabelItems } from "./lib/api/label";
import {
  addTodoItem,
  deleteTodoItem,
  getTodoItems,
  updateTodoItem,
} from "./lib/api/todo";
import {
  Label,
  NewLabelPayload,
  NewTodoPayload,
  Todo,
  UpdateTodoPayload,
} from "./types/todo";

const TodoApp: FC = () => {
  const [todos, setTodos] = useState<Todo[]>([]);
  const [labels, setLabels] = useState<Label[]>([]);
  const [filterLabelId, setFilterLabelId] = useState<number | null>(null);

  const onSubmit = async (payload: NewTodoPayload) => {
    if (!payload.text) {
      return;
    }
    await addTodoItem(payload);
    setTodos(await getTodoItems());
  };

  const onUpdate = async (updateTodo: UpdateTodoPayload) => {
    await updateTodoItem(updateTodo);
    setTodos(await getTodoItems());
  };

  const onDelete = async (id: number) => {
    await deleteTodoItem(id);
    setTodos(await getTodoItems());
  };

  const onSelectLabel = (label: Label | null) => {
    setFilterLabelId(label?.id ?? null);
  };

  const onSubmitNewLabel = async (newLabel: NewLabelPayload) => {
    if (!labels.some((label) => label.name === newLabel.name)) {
      setLabels([...labels, await addLabelItem(newLabel)]);
    }
  };

  const onDeleteLabel = async (id: number) => {
    await deleteLabelItem(id);
    setLabels((prev) => prev.filter((label) => label.id !== id));
  };

  const dispTodo = filterLabelId
    ? todos.filter((todo) =>
        todo.labels.some((label) => label.id === filterLabelId)
      )
    : todos;

  useEffect(() => {
    (async () => {
      setTodos(await getTodoItems());
      setLabels(await getLabelItems());
    })();
  }, []);

  return (
    <>
      <Box
        sx={{
          backgroundColor: "white",
          borderBottom: "1px solid gray",
          display: "flex",
          alignItems: "center",
          position: "fixed",
          top: 0,
          p: 2,
          width: "100%",
          height: 80,
          zIndex: 3,
        }}
      >
        <Typography variant="h1">Todo App</Typography>
      </Box>
      <Box
        sx={{
          backgroundColor: "white",
          borderRight: "1px solid gray",
          position: "fixed",
          height: "calc(100% - 80px)",
          width: 200,
          zIndex: 2,
          left: 0,
        }}
      >
        <SideNav
          labels={labels}
          onSelectLabel={onSelectLabel}
          filterLabelId={filterLabelId}
          onSubmitNewLabel={onSubmitNewLabel}
          onDeleteLabel={onDeleteLabel}
        />
      </Box>
      <Box
        sx={{
          display: "flex",
          justifyContent: "center",
          p: 5,
          mt: 10,
        }}
      >
        <Box maxWidth={700} width="100%">
          <Stack spacing={5}>
            <TodoForm onSubmit={onSubmit} labels={labels} />
            <TodoList
              todos={dispTodo}
              labels={labels}
              onUpdate={onUpdate}
              onDelete={onDelete}
            />
          </Stack>
        </Box>
      </Box>
    </>
  );
};

const theme = createTheme({
  typography: {
    h1: {
      fontSize: 30,
    },
    h2: {
      fontSize: 20,
    },
  },
});

const App: FC = () => {
  return (
    <ThemeProvider theme={theme}>
      <TodoApp />
    </ThemeProvider>
  );
};

export default App;
