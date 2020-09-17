package `var`

fun updateRequestHandler(request: UpdateRequest) {
    val user = readUser(request.email)
    user?.let {
        update(user, request)
        store(user)
    }
}

private fun update(user: User, request: UpdateRequest) {
    user.email = request.email
    user.name = request.name
}


private fun store(user: User) {
    // TODO call DB
}

private fun readUser(email: String): User? = when (email) {
    // TODO call DB
    "am@novatec-gmbh.de" -> User(
        email = email,
        name = "Alex"
    )
    else -> null
}

data class UpdateRequest(
    val email: String,
    val name: String
)

private data class User(
    var email: String,
    var name: String
)
