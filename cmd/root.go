package cmd

import (
	"fmt"
	"os"

	bubbleTeaCmd "github.com/roloenusa/cli-ent/cmd/bubbletea"
	surveyCmd "github.com/roloenusa/cli-ent/cmd/survey"
	"github.com/spf13/cobra"
)

var rootCmd = &cobra.Command{
	Use:   "cmd",
	Short: "",
	Long:  ``,
	// Run: func(cmd *cobra.Command, args []string) {

	// },
}

func Execute() {
	rootCmd.AddCommand(surveyCmd.SurveyCmd)
	rootCmd.AddCommand(bubbleTeaCmd.BubbleTeaCmd)

	if err := rootCmd.Execute(); err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
	}
}
