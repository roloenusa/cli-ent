package prompt

import (
	"fmt"

	"github.com/charmbracelet/lipgloss"
)

var style = lipgloss.NewStyle().
	Bold(true).
	Foreground(lipgloss.Color("#FAFAFA")).
	Background(lipgloss.Color("#7D56F4")).
	PaddingLeft(4).
	PaddingRight(4)

func Question(question string) string {
	return fmt.Sprintf("%s\n\n", style.Render(question))
}
