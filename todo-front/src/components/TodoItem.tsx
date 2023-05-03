import {
  Box,
  Button,
  Card,
  Checkbox,
  Chip,
  FormControlLabel,
  Grid,
  Modal,
  Stack,
  TextField,
  Typography,
} from "@mui/material";
import { ChangeEventHandler, FC, useEffect, useState } from "react";
import { toggleLabels } from "../lib/toggleLabels";
import { modalInnerStyle } from "../styles/modal";
import { Label, Todo, UpdateTodoPayload } from "../types/todo";

type Props = {
  todo: Todo;
  onUpdate: (todo: UpdateTodoPayload) => void;
  onDelete: (id: number) => void;
  labels: Label[];
};

const TodoItem: FC<Props> = ({ todo, onUpdate, onDelete, labels }) => {
  const [editing, setEditing] = useState(false);
  const [editText, setEditText] = useState("");
  const [editLabels, setEditLabels] = useState<Label[]>([]);

  useEffect(() => {
    setEditText(todo.text);
    setEditLabels(todo.labels);
  }, [todo, editing]);

  const handleCompletedCheckbox: ChangeEventHandler = (_e) => {
    onUpdate({
      ...todo,
      completed: !todo.completed,
      labels: todo.labels.map((label) => label.id),
    });
  };

  const onCloseEditModal = () => {
    onUpdate({
      ...todo,
      text: editText,
      completed: todo.completed,
      labels: editLabels.map((label) => label.id),
    });
    setEditing(false);
  };

  const handleDelete = () => onDelete(todo.id);

  return (
    <Card sx={{ p: 1 }}>
      <Grid container spacing={2} alignItems="center">
        <Grid item xs={1}>
          <Checkbox
            onChange={handleCompletedCheckbox}
            checked={todo.completed}
          />
        </Grid>
        <Grid item xs={8}>
          <Stack spacing={1}>
            <Typography variant="caption" fontSize={16}>
              {todo.text}
            </Typography>
            <Stack direction="row" spacing={1}>
              {todo.labels?.map((label) => (
                <Chip key={label.id} label={label.name} size="small" />
              ))}
            </Stack>
          </Stack>
        </Grid>
        <Grid item xs={2}>
          <Stack direction="row" spacing={1}>
            <Button onClick={() => setEditing(true)} color="info">
              edit
            </Button>
            <Button onClick={handleDelete} color="error">
              delete
            </Button>
          </Stack>
        </Grid>
      </Grid>
      <Modal open={editing} onClose={onCloseEditModal}>
        <Box sx={modalInnerStyle}>
          <Stack spacing={2}>
            <TextField
              size="small"
              label="todo text"
              defaultValue={todo.text}
              onChange={(e) => setEditText(e.target.value)}
            />
            <Stack>
              <Typography variant="subtitle1">Labels</Typography>
              {labels.map((label) => (
                <FormControlLabel
                  key={label.id}
                  control={
                    <Checkbox
                      defaultChecked={todo.labels.some(
                        (todoLabel) => todoLabel.id === label.id
                      )}
                    />
                  }
                  label={label.name}
                  onChange={() =>
                    setEditLabels((prev) => toggleLabels(prev, label))
                  }
                />
              ))}
            </Stack>
          </Stack>
        </Box>
      </Modal>
    </Card>
  );
};

export default TodoItem;
