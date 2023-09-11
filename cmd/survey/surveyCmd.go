package surveyCmd

import (
	"fmt"
	"os"

	"github.com/roloenusa/cli-ent/pkg/survey"
	"github.com/spf13/cobra"
)

var SurveyCmd = &cobra.Command{
	Use:   "survey",
	Short: "Run a prompt cli with survey",
	Long:  ``,
	Run: func(cmd *cobra.Command, args []string) {
		survey.Execute()
	},
}

func Execute() {

	if err := SurveyCmd.Execute(); err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
	}
}
