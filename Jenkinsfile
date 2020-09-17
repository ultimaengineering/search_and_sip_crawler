podTemplate(yaml: """
apiVersion: v1
kind: Pod
metadata:
  labels:
    some-label: builder
spec:
  containers:
  - name: rust
    image: rust:1.43-stretch
    command:
    - cat
    tty: true
"""
  ) {
  node(POD_LABEL) {
        stage('Build and test') {
          checkout scm
          container('rust') {
            sh 'cargo test'
            sh 'cargo build --release'
          }
        }
}}