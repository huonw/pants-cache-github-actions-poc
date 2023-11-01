for i in range(100):
    shell_command(
        name=f"files-{i}",
        command=f"""
        mkdir out-{i}
        for j in $(seq 1 100); do
           echo "{i} $j" > "out-{i}/file-$j"
        done
        sleep 1
        """,
        tools=["sleep", "mkdir", "seq"],
        output_directories=[f"out-{i}"]
    )

    experimental_test_shell_command(
        name=f"test-{i}",
        command="sleep 1",
        tools=["sleep"],
        execution_dependencies=[f":files-{i}"]
    )
