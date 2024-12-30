# Repository APis

 Team User Access

| 0    | 1    | 3     | 6     | 7     | 9           |
|------|------|-------|-------|-------|-------------|
| None | Read | Write | Admin | Owner | SystemAdmin |


- [x] /api/v1/repo [ POST ]
- [x] /api/v1/repo/search/:query [ GET ]
- [x] /api/v1/repo/:owner/:name [ GET ]
- [x] /api/v1/repo/:owner/:name/branch [ GET ]
- [x] /api/v1/repo/:owner/:name/default_branch [ GET / POST ]
- [x] /api/v1/repo/:owner/:name/branch/:branch [ GET / DELETE ]
- [x] /api/v1/repo/:owner/:name/branch/:branch/protect [ POST ]
- [x] /api/v1/repo/:owner/:name/branch/:branch/unprotect [ POST ]
- [x] /api/v1/repo/:owner/:name/branch/:branch/blob [ GET ] TODO [  PUT / DELETE / POST ]
- [x] /api/v1/repo/:owner/:name/branch/:branch/commits [ GET ]
- [x] /api/v1/repo/:owner/:name/branch/:branch/commits/:sha [ GET ]
- [x] /api/v1/repo/:owner/:name/branch/:branch/commits/:sha/files [ GET ]
- [ ] /api/v1/repo/:owner/:name/branch/:branch/commits/:sha/status [ GET ]
- [x] /api/v1/repo/:owner/:name/branch/:branch/trees [ GET ]
- [x] /api/v1/repo/:owner/:name/branch/:branch/trees/:sha [ GET ]
- [x] /api/v1/repo/:owner/:name/star [ POST / GET / DELETE / PATCH ]
- [x] /api/v1/repo/:owner/:name/watch/:level [ POST / PUT ]
- [x] /api/v1/repo/:owner/:name/watch [  GET / PATCH / DELETE ]
- [ ] /api/v1/repo/:owner/:name/fork [ POST / GET ]

- [ ] /api/v1/repo/:owner/:name/collaborators [ GET ]
- [ ] /api/v1/repo/:owner/:name/collaborators/:username [ GET / POST / DELETE / PATCH ]
- [ ] /api/v1/repo/:owner/:name/collaborators/:username/permission [ GET / POST / DELETE / PATCH ]

- [ ] /api/v1/repo/:owner/:name/issues [ GET / POST ]
- [ ] /api/v1/repo/:owner/:name/issues/:id [ GET / POST / PATCH ]
- [ ] /api/v1/repo/:owner/:name/issues/:id/comments [ GET / POST ]
- [ ] /api/v1/repo/:owner/:name/issues/:id/comments/:id [ GET / POST / PATCH ]
- [ ] /api/v1/repo/:owner/:name/issues/:id/labels [ GET / POST ]
- [ ] /api/v1/repo/:owner/:name/issues/:id/labels/:id [ GET / POST / PATCH ]
- [ ] /api/v1/repo/:owner/:name/issues/:id/assignees [ GET / POST ]
- [ ] /api/v1/repo/:owner/:name/issues/:id/assignees/:id [ GET / POST / PATCH ]
- [ ] /api/v1/repo/:owner/:name/issues/:id/milestone [ GET / POST / PATCH ]
- [ ] /api/v1/repo/:owner/:name/issues/:id/milestones [ GET ]
- [ ] /api/v1/repo/:owner/:name/issues/:id/milestones/:id [ GET / POST / PATCH ]
- [ ] /api/v1/repo/:owner/:name/issues/:id/reactions [ GET / POST ]
- [ ] /api/v1/repo/:owner/:name/issues/:id/reactions/:id [ GET / POST / PATCH ]
- [ ] /api/v1/repo/:owner/:name/issues/:id/reactions/:id/users [ GET ]

- [ ] /api/v1/repo/:owner/:name/pulls [ GET / POST ]
- [ ] /api/v1/repo/:owner/:name/pulls/:id [ GET / POST / PATCH ]
- [ ] /api/v1/repo/:owner/:name/pulls/:id/comments [ GET / POST ]
- [ ] /api/v1/repo/:owner/:name/pulls/:id/comments/:id [ GET / POST / PATCH ]
- [ ] /api/v1/repo/:owner/:name/pulls/:id/labels [ GET / POST ]
- [ ] /api/v1/repo/:owner/:name/pulls/:id/labels/:id [ GET / POST / PATCH ]
- [ ] /api/v1/repo/:owner/:name/pulls/:id/assignees [ GET / POST ]
- [ ] /api/v1/repo/:owner/:name/pulls/:id/assignees/:id [ GET / POST / PATCH ] 
- [ ] /api/v1/repo/:owner/:name/pulls/:id/milestone [ GET / POST / PATCH ]
- [ ] /api/v1/repo/:owner/:name/pulls/:id/milestones [ GET ]

- [ ] /api/v1/repo/:owner/:name/releases [ GET / POST ]
- [ ] /api/v1/repo/:owner/:name/releases/:id [ GET / POST / PATCH ]

- [ ] /api/v1/repo/:owner/:name/reviews [ GET / POST ]
- [ ] /api/v1/repo/:owner/:name/reviews/:id [ GET / POST / PATCH ]

- [ ] /api/v1/repo/:owner/:name/tags [ GET / POST ]
- [ ] /api/v1/repo/:owner/:name/tags/:id [ GET / POST / PATCH ]

- [ ] /api/v1/repo/:owner/:name/topics [ GET / POST ]

- [ ] /apo/v1/repo/:owner/:name/branches [ GET ]