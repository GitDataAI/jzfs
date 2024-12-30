# UserAPis 

- [x] /api/v1/users/login [ POST ]

| email  | password |
|--------|----------|
| string | string   |
Use Base64


- [x] /api/v1/users/apply [ POST ]

| email  | password | username |
|--------|----------|----------|
| string | string   | string   |

- [x] /api/v1/users/logout [ POST ]


- [ ] /api/v1/user/refresh [ POST ]

- [x] /api/v1/user [ GET  ]

- [x] /api/v1/user/avatar [ PUT / GET ]

- [ ] /api/v1/user/option [ POST / GET ]

- [x] /api/v1/user/ssh_key [ POST / GET  ]
- [x] /api/v1/user/ssh_key/{uid} [ GET / DELETE ]

- [ ] /api/v1/user/token [ POST / GET ]
- [ ] /api/v1/user/token/{id} [ GET / DELETE ]
- [ ] /api/v1/user/token/refresh [ POST ]

- [ ] /api/v1/user/password [ POST ]

- [x] /api/v1/user/followers [ GET ]
- [x] /api/v1/user/following [ GET ]
- [x] /api/v1/user/following/{username} [ POST / DELETE ]
- [x] /api/v1/user/following/count [ GET ]
- [x] /api/v1/user/followers/count [ GET ]

- [ ] /api/v1/user/notifications [ GET ]
- [ ] /api/v1/user/notifications/{id} [ GET / DELETE ]
- [ ] /api/v1/user/notifications/count [ GET ]

- [x] /api/v1/user/email [ GET / POST / DELETE ]
- [ ] /api/v1/user/email/verify [ POST ]
- [ ] /api/v1/user/email/resend [ POST ]

- [ ] /api/v1/user/repo [ GET / POST ]
- [ ] /api/v1/user/repo/{repo} [ GET / DELETE ]

- [x] /api/v1/users/search [ GET ]
- [x] /api/v1/users/information/{username} [ GET ]
- [x] /api/v1/users/setting [ GET ]