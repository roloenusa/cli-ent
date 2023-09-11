package btea

import (
	"fmt"
	"os"

	tea "github.com/charmbracelet/bubbletea"
	"github.com/roloenusa/cli-ent/pkg/btea/prompt"
)

func Execute() {
	m := prompt.InitialModel([]string{"Test 1", "Test 2"})
	p := tea.NewProgram(m)
	if _, err := p.Run(); err != nil {
		fmt.Printf("Alas, there's been an error: %v", err)
		os.Exit(1)
	}

	n := prompt.InitialModel([]string{"Test A", "Test B"})
	s := tea.NewProgram(n)
	if _, err := s.Run(); err != nil {
		fmt.Printf("Alas, there's been an error: %v", err)
		os.Exit(1)
	}
}
