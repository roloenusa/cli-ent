package prompt

import (
	tea "github.com/charmbracelet/bubbletea"
)

type model struct {
	choices  []string         // items on the to-do list
	cursor   int              // which to-do list item our cursor is pointing at
	selected map[int]struct{} // which to-do items are selected
	done     bool             // Done with the model
}

func InitialModel(choices []string) model {
	return model{
		// Our to-do list is a grocery list
		choices: choices,

		// A map which indicates which choices are selected. We're using
		// the  map like a mathematical set. The keys refer to the indexes
		// of the `choices` slice, above.
		selected: make(map[int]struct{}),
	}
}

func (m model) Init() tea.Cmd {
	// Just return `nil`, which means "no I/O right now, please."
	return nil
}

func (m model) Update(msg tea.Msg) (tea.Model, tea.Cmd) {
	switch msg := msg.(type) {

	// Is it a key press?
	case tea.KeyMsg:

		// Cool, what was the actual key pressed?
		switch msg.String() {

		// These keys should exit the program.
		case "ctrl+c", "q":
			return m, tea.Quit

		// The "up" and "k" keys move the cursor up
		case "up", "k":
			if m.cursor > 0 {
				m.cursor--
			}

		// The "down" and "j" keys move the cursor down
		case "down", "j":
			if m.cursor < len(m.choices)-1 {
				m.cursor++
			}

		// The "enter" key and the spacebar (a literal space) toggle
		// the selected state for the item that the cursor is pointing at.
		case " ":
			_, ok := m.selected[m.cursor]
			if ok {
				delete(m.selected, m.cursor)
			} else {
				m.selected[m.cursor] = struct{}{}
			}

		// The "enter" key and the spacebar (a literal space) toggle
		// the selected state for the item that the cursor is pointing at.
		case "enter":
			m.done = true
			return m, tea.Quit
		}
	}

	// Return the updated model to the Bubble Tea runtime for processing.
	// Note that we're not returning a command.
	return m, nil
}

func (m model) View() string {
	// The header
	s := Question("What should we buy at the market?")

	if !m.done {

		// Iterate over our choices
		for i, choice := range m.choices {

			// Is the cursor pointing at this choice?
			cursor := false // no cursor
			if m.cursor == i {
				cursor = true // cursor!
			}

			// Is this choice selected?
			checked := false // not selected
			if _, ok := m.selected[i]; ok {
				checked = true // selected!
			}

			// Render the row
			s += Choice(cursor, checked, choice)
		}

		// The footer
		s += "\nPress q to quit.\n"
	} else {
		// Iterate over our choices
		for i, choice := range m.choices {
			if _, ok := m.selected[i]; ok {
				// Render the row
				// s += fmt.Sprintf("%s\n", choice)
				s += Answer(choice)
			}
		}
		s += "\n"
	}

	// Send the UI for rendering
	return s
}
