const fs = require("node:fs/promises");
const { v4: uuidv4 } = require("uuid");
const { faker } = require("@faker-js/faker");

function getWeekNumber(d) {
  // Copy date so don't modify original
  d = new Date(Date.UTC(d.getFullYear(), d.getMonth(), d.getDate()));
  // Set to nearest Thursday: current date + 4 - current day number
  // Make Sunday's day number 7
  d.setUTCDate(d.getUTCDate() + 4 - (d.getUTCDay() || 7));
  // Get first day of year
  var yearStart = new Date(Date.UTC(d.getUTCFullYear(), 0, 1));
  // Calculate full weeks to nearest Thursday
  var weekNo = Math.ceil(((d - yearStart) / 86400000 + 1) / 7);
  return weekNo;
}

const currentYear = new Date().getFullYear();
const currentWeek = getWeekNumber(new Date());

async function writeToFile(folder, file, num, dataJSON) {
  try {
    await fs.writeFile(
      `./src/.mock/${folder}/${file}${num + 1}.json`,
      dataJSON
    );
    console.log(`File has been created for ${file}`);
  } catch (err) {
    console.log(err);
  }
}

const time = [
  "13",
  "15",
  "17",
  "18",
  "13-17",
  "15-17",
  "(17)",
  "(18)",
  "(17)18",
  "available",
  "free",
];

const weekDays = [
  "monday",
  "tuesday",
  "wednesday",
  "thursday",
  "friday",
  "saturday",
  "sunday",
];

const roles = [
  "griller",
  "kitchen",
  "bar",
  "service",
  "management",
  "dishwasher",
  "none",
  "all",
];

for (let n = 0; n < 10; n++) {
  let ID = uuidv4();
  let name = faker.person.firstName();
  let maxNum = Math.floor(Math.random() * 5);
  let roleNum = Math.floor(Math.random() * roles.length);
  let roleSecNum = Math.floor(Math.random() * roles.length);

  for (let i = 0; i < 7; i++) {
    const userPlan = {
      user_id: ID,
      weekly_id: `${currentYear}-${currentWeek}`,
      name: name,
      day: weekDays[i],
      time: time[Math.floor(Math.random() * time.length)],
    };

    const dataPlan = JSON.stringify(userPlan, null, 2);
    writeToFile("plan", "plan", n * 7 + i, dataPlan);
  }

  // Define some sample data as an object
  const userData = {
    id: ID,
    name: name,
    employee_id: `${n}`,
    admin: "false",
    vast: "false",
    active: "true",
    min_days: `${maxNum - 1 < 0 ? 1 : maxNum - 1}`,
    max_days: `${maxNum}`,
    role_primary:
      roles[roleNum] == "all"
        ? "service"
        : roles[roleNum] == "none"
        ? "service"
        : roles[roleNum],
    role_secondary:
      roles[roleSecNum] == "management" ? "all" : roles[roleSecNum],
  };

  const dataJSON = JSON.stringify(userData, null, 2);
  // Use the fs.writeFile function to create the file
  writeToFile("users", "user", n, dataJSON);
}
