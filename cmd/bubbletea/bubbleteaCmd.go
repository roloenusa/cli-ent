package bubbleteaCmd

import (
	"fmt"
	"os"

	"github.com/roloenusa/cli-ent/pkg/btea"
	"github.com/spf13/cobra"
)

var BubbleTeaCmd = &cobra.Command{
	Use:   "bubbletea",
	Short: "Run a prompt cli with bubbletea",
	Long:  ``,
	Run: func(cmd *cobra.Command, args []string) {
		btea.Execute()
	},
}

func Execute() {

	if err := BubbleTeaCmd.Execute(); err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
	}
}
