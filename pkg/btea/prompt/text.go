package prompt

import (
	"fmt"

	"github.com/charmbracelet/lipgloss"
)

func Question(question string) string {
	var style = lipgloss.NewStyle().
		Bold(true).
		Foreground(lipgloss.Color("#FAFAFA")).
		Background(lipgloss.Color("#7D56F4")).
		PaddingLeft(4).
		PaddingRight(4)
	return fmt.Sprintf("%s\n\n", style.Render(question))
}

func Answer(answer string) string {
	var style = lipgloss.NewStyle().
		Foreground(lipgloss.Color("#888888")).
		PaddingLeft(4)
	return fmt.Sprintf("%s\n", style.Render(answer))
}

func Choice(cursor bool, checked bool, choice string) string {
	var style = lipgloss.NewStyle()

	var cursorStyle = lipgloss.NewStyle().
		Bold(true).
		Foreground(lipgloss.Color("#7D56F4"))

	cursorStr := " "
	if cursor {
		cursorStr = ">"
	}

	var checkboxStyle = lipgloss.NewStyle().
		Bold(true).
		Foreground(lipgloss.Color("#31C445"))

	checkboxStr := "[ ]"
	if checked {
		checkboxStr = "[X]"
	}

	return fmt.Sprintf("%s %s %s\n", cursorStyle.Render(cursorStr), checkboxStyle.Render(checkboxStr), style.Render(choice))
}
