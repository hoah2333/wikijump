const readline = require("readline")
const { execSync, spawn } = require("child_process")
const pc = require("picocolors")

// technically never gets cleared, but it's not a big deal
// if our CLI spawns so many processes that it becomes a problem
// I'll be impressed
/** @type Set<import("child_process").ChildProcessWithoutNullStreams> */
const processes = new Set()

function linebreak() {
  console.log("")
}

function separator() {
  console.log(pc.gray("─────────────────────────"))
}

function section(title, breakBefore = false, breakAfter = breakBefore) {
  if (breakBefore) linebreak()
  const dashes = Math.round((24 - (title.length + 2)) / 2)
  const chrs = "─".repeat(dashes)
  console.log(pc.blue(`${chrs} ${title} ${chrs}`))
  if (breakAfter) linebreak()
}

function info(...msgs) {
  const msg = msgs.join("\n")
  console.log(pc.green(msg))
}

function infoline(...msgs) {
  linebreak()
  const msg = msgs.join("\n")
  console.log(pc.green(msg))
  linebreak()
}

function look(...msgs) {
  const msg = msgs.join("\n")
  console.log(pc.magenta(msg))
}

function warn(...msgs) {
  const msg = msgs.join("\n")
  console.warn(pc.yellow("──────── WARNING ────────"))
  console.warn(pc.yellow(msg))
}

function error(...msgs) {
  const msg = msgs.join("\n")
  console.warn(pc.red("───────── ERROR ─────────"))
  console.warn(pc.red(msg))
}

function cmd(command, pipe = true) {
  return execSync(command, pipe ? { stdio: "inherit" } : {})
}

/** @returns {Promise<Buffer>} */
function cmdAsync(command, pipe = true) {
  return new Promise(resolve => {
    const child = spawn(command, { shell: true, stdio: pipe ? "inherit" : undefined })
    processes.add(child)
    child.on("close", () => resolve(child.stdout))
  })
}

function shell(command, pipe = true) {
  const child = spawn(command, { shell: true, stdio: pipe ? "inherit" : undefined })
  processes.add(child)
  return child
}

/** @returns {Promise<import("child_process").ChildProcess>} */
function shellAsync(command, pipe = true) {
  return new Promise(resolve => {
    const child = spawn(command, { shell: true, stdio: pipe ? "inherit" : undefined })
    processes.add(child)
    child.on("spawn", () => resolve(child))
  })
}

function question(question) {
  return new Promise((resolve, reject) => {
    const rl = readline.createInterface({
      input: process.stdin,
      output: process.stdout
    })

    rl.question(pc.magenta(question), answer => {
      // breaks using readline for anything else
      // so we'll just leave the interface alone (technically a memory leak)
      // rl.close()
      resolve(answer)
    })
  })
}

function answerYesOrNo(answer, def = false) {
  answer = answer.toLowerCase().trim()
  if (answer === "y" || answer === "yes") return true
  if (answer === "n" || answer === "no") return false
  return def
}

module.exports = {
  pc,
  processes,
  linebreak,
  separator,
  section,
  info,
  infoline,
  look,
  warn,
  error,
  cmd,
  cmdAsync,
  shell,
  shellAsync,
  question,
  answerYesOrNo
}
