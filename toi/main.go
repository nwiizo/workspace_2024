package main

import (
	"bufio"
	"bytes"
	"fmt"
	"io"
	"os"
	"strings"
	"time"

	"github.com/spf13/cobra"
)

var (
	timeout      int
	defaultYes   bool
	defaultNo    bool
	customPrompt string
)

func main() {
	if err := rootCmd.Execute(); err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
	}
}

var rootCmd = &cobra.Command{
	Use:   "toi",
	Short: "toi is a pipeline confirmation tool",
	Long: `toi is a tool that allows you to confirm actions in a pipeline.
It displays the input content and prompts for confirmation before proceeding.`,
	RunE: run,
}

func init() {
	rootCmd.Flags().IntVarP(&timeout, "timeout", "t", 0, "Timeout in seconds (0 for no timeout)")
	rootCmd.Flags().BoolVarP(&defaultYes, "yes", "y", false, "Default to yes if no input is provided")
	rootCmd.Flags().BoolVarP(&defaultNo, "no", "n", false, "Default to no if no input is provided")
	rootCmd.Flags().StringVarP(&customPrompt, "prompt", "p", "", "Custom prompt message")
}

func run(cmd *cobra.Command, args []string) error {
	inputChan := make(chan []byte)
	errChan := make(chan error)

	go func() {
		input, err := io.ReadAll(os.Stdin)
		if err != nil {
			errChan <- fmt.Errorf("error reading input: %w", err)
			return
		}
		inputChan <- input
	}()

	var input []byte
	select {
	case input = <-inputChan:
	case err := <-errChan:
		return err
	}

	if err := displayInput(input); err != nil {
		return err
	}

	proceed, err := shouldProceed()
	if err != nil {
		return err
	}

	if proceed {
		return writeOutput(input)
	}

	fmt.Fprintln(os.Stderr, "Operation cancelled.")
	return nil
}

func displayInput(input []byte) error {
	_, err := fmt.Fprintln(os.Stderr, "Input content:")
	if err != nil {
		return fmt.Errorf("error writing to stderr: %w", err)
	}
	_, err = fmt.Fprintln(os.Stderr, string(input))
	if err != nil {
		return fmt.Errorf("error writing to stderr: %w", err)
	}
	return nil
}

func shouldProceed() (bool, error) {
	prompt := "Do you want to proceed? (y/n): "
	if customPrompt != "" {
		prompt = customPrompt
	}

	_, err := fmt.Fprint(os.Stderr, prompt)
	if err != nil {
		return false, fmt.Errorf("error writing prompt to stderr: %w", err)
	}

	response, err := getResponse()
	if err != nil {
		return false, err
	}

	response = strings.TrimSpace(strings.ToLower(response))

	if response == "" {
		if defaultYes {
			fmt.Fprintln(os.Stderr, "Using default: yes")
			return true, nil
		} else if defaultNo {
			fmt.Fprintln(os.Stderr, "Using default: no")
			return false, nil
		}
	}

	return response == "y" || response == "yes", nil
}

func getResponse() (string, error) {
	if timeout > 0 {
		return getResponseWithTimeout(timeout)
	}

	return readFromTTY()
}

func getResponseWithTimeout(seconds int) (string, error) {
	ch := make(chan string)
	errCh := make(chan error)

	go func() {
		response, err := readFromTTY()
		if err != nil {
			errCh <- err
			return
		}
		ch <- response
	}()

	select {
	case response := <-ch:
		return response, nil
	case err := <-errCh:
		return "", err
	case <-time.After(time.Duration(seconds) * time.Second):
		_, err := fmt.Fprintln(os.Stderr, "\nTimeout reached. Using default.")
		if err != nil {
			return "", fmt.Errorf("error writing timeout message: %w", err)
		}
		return "", nil
	}
}

func readFromTTY() (string, error) {
	tty, err := os.Open("/dev/tty")
	if err != nil {
		return "", fmt.Errorf("failed to open /dev/tty: %w", err)
	}
	defer tty.Close()

	reader := bufio.NewReader(tty)
	response, err := reader.ReadString('\n')
	if err != nil {
		return "", fmt.Errorf("error reading from tty: %w", err)
	}

	return strings.TrimSpace(response), nil
}

func writeOutput(input []byte) error {
	buffer := bytes.NewBuffer(input)
	_, err := io.Copy(os.Stdout, buffer)
	if err != nil {
		return fmt.Errorf("error writing to stdout: %w", err)
	}
	return nil
}
