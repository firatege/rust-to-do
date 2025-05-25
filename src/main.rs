use chrono::NaiveTime;
use std::io;


#[derive(Clone, Debug)]
enum UserRole {
    Classic,
    Vip,
    Admin,
    Owner
}
#[derive(Clone, Debug)]
struct User {
    id: u32, // 32bit number
    username: String,
    email: String,
    is_active: bool,
    user_role: UserRole,
}
#[derive(Clone, Debug)]
struct WorkSpace {
    id: u32,
    description: String,
    owner: User,
    create_at: NaiveTime,
    tables: Vec<GoalTable>,
}
#[derive(Clone, Debug)]
struct GoalTable {
    id: u32,
    title: String,
    description: String,
    owner: User,
    workspace: u32, // workspace id
    create_at: NaiveTime,
    goals: Vec<Goal>,
    is_active: bool,
}
#[derive(Clone, Debug)]
struct Goal {
    id: u32,
    owner: User,
    text: String,
    table: u32, // table id
    create_at: NaiveTime,
    is_active: bool,
}


impl User {
    fn new(id:u32,username:String,email:String,user_role:UserRole) -> User {
        User {
            id,
            username,
            email,
            is_active: true,
            user_role
        }
    }

    fn display_info(&self) {
        println!("User ID: {}",self.id);
        println!("Username: {}",self.username);
        println!("Email: {}",self.email);
        println!("Is Active: {}",self.is_active);
    }
}

impl WorkSpace {
    fn new(id:u32, description:String, owner:User) -> WorkSpace {
        WorkSpace {
            id,
            description,
            owner,
            create_at: NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
            tables: Vec::new()
        }
    }

    fn add_table(&mut self, table: GoalTable) {
        self.tables.push(table);
    }

    fn display_info(&self) {
        println!("Workspace ID: {}", self.id);
        println!("Description: {}", self.description);
        println!("Owner: {}", self.owner.username);
        println!("Tables:");
        for table in &self.tables {
            println!("- {}", table.title);
        }
    }
}

impl GoalTable {
    fn new(id:u32, title:String, description:String, owner:User, workspace:u32) -> GoalTable {
        GoalTable {
            id,
            title,
            description,
            owner,
            workspace,
            create_at: NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
            goals: Vec::new(),
            is_active: true
        }
    }

    fn add_goal(&mut self, goal: Goal) {
        self.goals.push(goal);
    }

    fn display_info(&self) {
        println!("Table Title: {}", self.title);
        println!("Description: {}", self.description);
        println!("Owner: {}", self.owner.username);
        println!("Is Active: {}", self.is_active);
        println!("Goals:");
        for goal in &self.goals {
            println!("- {}", goal.text);
        }
    }
}

impl Goal {
    fn new(id:u32, owner:User, text:String, table:u32) -> Goal {
        Goal {
            id,
            owner,
            text,
            table,
            create_at: NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
            is_active: true
        }
    }

    fn display_info(&self) {
        println!("Goal Text: {}", self.text);
        println!("Owner: {}", self.owner.username);
        println!("Is Active: {}", self.is_active);
    }
}




fn try_create_user(id: u32, username: String, email: String, user_role: UserRole) -> Result<User, String> {
    if username.trim().is_empty() {
        return Err("Kullanıcı adı boş olamaz.".to_string());
    }
    if !(email.contains('@') && email.contains('.')) {
        return Err("Geçerli bir e-posta giriniz.".to_string());
    }
    Ok(User::new(id, username, email, user_role))
}


fn try_create_goal(id: u32, owner: User, text: String, table: u32) -> Result<Goal, String> {
    if text.trim().is_empty() {
        return Err("Hedef metni boş olamaz.".to_string());
    }
    Ok(Goal::new(id, owner, text, table))
}

fn try_create_goal_table(id:u32, title:String, description:String, owner:User, workspace:u32) -> Result<GoalTable,String> {
    if title.trim().is_empty() {
        return Err("Target Title is not be empty".to_string());
    } else if description.trim().is_empty() {
        return Err("Target description is not be empty".to_string());
    }
    Ok(GoalTable::new(id, title, description, owner, workspace))
}

fn try_create_workspace(id:u32, description:String, owner:User) -> Result<WorkSpace,String> {
    if description.trim().is_empty() {
        return Err("Target description is not be empty".to_string());
    }
    Ok(WorkSpace::new(id, description, owner))
}

fn get_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Girdi okunamadı.");
    input.trim().to_string()
}

fn main() {

    let mut choice = String::new();
    let mut user_id_counter = 1; 
    let mut workspace_id_counter = 1; 
    let mut goal_table_id_counter = 1;


    let mut current_user: Option<User> = None;
    let mut current_workspace: Option<WorkSpace> = None;
    let mut current_goal_table: Option<GoalTable> = None;
    let mut current_goal: Option<Goal> = None;
    println!("Welcome to the Goal Management System!");


 
 loop {
        println!("\n\n\n");
        println!("1. Test Environment");
        println!("2. Create User");
        println!("3. Create Goal");
        println!("4. Create Goal Table");
        println!("5. Create Workspace");
        println!("6. Print All");
        println!("7. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice: u32 = choice.trim().parse().expect("Please enter a number");

        match choice {
            1 => {
                // Test Environment
                let user = User::new(1, "test_user".to_string(), "test_user@gmail.com".to_string(), UserRole::Classic);
                let goal = Goal::new(1, user.clone(), "Test Goal".to_string(), 1);
                let mut goal_table = GoalTable::new(1, "Test Table".to_string(), "Test Description".to_string(), user.clone(), 1);
                let mut workspace = WorkSpace::new(1, "Test Workspace".to_string(), user.clone());
                workspace.add_table(goal_table.clone());
                goal_table.add_goal(goal.clone());
                print!("{}[2J", 27 as char);
                println!("Test Environment Created:");
                println!("User: {:?}", user);
                println!("Goal: {:?}", goal);
                println!("Goal Table: {:?}", goal_table);
                println!("Workspace: {:?}", workspace);
            }
            2 => {
                // Create User
                print!("{}[2J", 27 as char);
                let username = get_input("Enter Username:");
                let email = get_input("Enter Email:");
                let user_role = get_input("Enter User Role (Classic, Vip, Admin, Owner):");
                let user_role = match user_role.as_str() {
                    "Classic" => UserRole::Classic,
                    "Vip" => UserRole::Vip,
                    "Admin" => UserRole::Admin,
                    "Owner" => UserRole::Owner,
                    _ => UserRole::Classic,
                };
                match try_create_user(user_id_counter, username.clone(), email.clone(), user_role.clone()) {
                    Ok(user) => {
                        current_user = Some(user);
                        user_id_counter += 1;
                        println!("User created successfully!");
                
                    }
                    Err(e) => println!("Error creating user: {}", e),
                }
            }
            3 => {
                // Create Goal
                print!("{}[2J", 27 as char);
                if let Some(user) = &current_user {
                    if let Some(table) = &current_goal_table {
                        let text = get_input("Enter Goal Text:");
                        match try_create_goal(goal_table_id_counter, user.clone(), text.clone(), table.id) {
                            Ok(goal) => {
                                println!("Goal created successfully!");
                                goal.display_info();
                                goal_table_id_counter += 1;
                                current_goal = Some(goal.clone());
                            }
                            Err(e) => println!("Error creating goal: {}", e),
                        }
                    } else {
                        println!("Please create a Goal Table first.");
                    }
                } else {
                    println!("Please create a user first.");
                }
            }
            4 => {
                // Create Goal Table
                print!("{}[2J", 27 as char);
                if let Some(user) = &current_user {
                    if let Some(workspace) = &current_workspace {
                        let title = get_input("Enter Goal Table Title:");
                        let description = get_input("Enter Goal Table Description:");
                        match try_create_goal_table(goal_table_id_counter, title.clone(), description.clone(), user.clone(), 1) {
                            Ok(goal_table) => {
                                println!("Goal Table created successfully!");
                                goal_table.display_info();
                                goal_table_id_counter += 1;
                                current_goal_table = Some(goal_table.clone());
                                
                            }
                            Err(e) => println!("Error creating goal table: {}", e),
                        }
                    } else {
                        println!("Please create a WorkSpace first.");
                    }
                } else {
                    println!("Please create a user first.");
                }
            }
            5 => {
                // Create Workspace
                print!("{}[2J", 27 as char);
                if let Some(user) = &current_user {
                    let description = get_input("Enter Workspace Description:");
                    match try_create_workspace(workspace_id_counter, description.clone(), user.clone()) {
                        Ok(workspace) => {
                            println!("Workspace created successfully!");
                            workspace.display_info();
                            workspace_id_counter += 1;
                            current_workspace = Some(workspace);
                        }
                        Err(e) => println!("Error creating workspace: {}", e),
                    }
                } else {
                    println!("Please create a user first.");
                }
            }
            6 => {
                // Print All
                print!("{}[2J", 27 as char);
                if let Some(user) = &current_user {
                    user.display_info();
                } else {
                    println!("No user created.");
                }
                if let Some(workspace) = &current_workspace {
                    workspace.display_info();
                } else {
                    println!("No workspace created.");
                }
                if let Some(table) = &current_goal_table {
                    table.display_info();
                } else {
                    println!("No goal table created.");
                }
                if let Some(goal) = &current_goal {
                    goal.display_info();
                } else {
                    println!("No goal created.");
                }
            }
            7 => {
                // Exit
                print!("{}[2J", 27 as char);
                println!("Exiting...");
                break;
            }
            _ => println!("Invalid choice, please try again."),
        }
    }
}
         
                