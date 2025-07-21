DataLad is a Python-based tool for the joint management of code, data, and their relationship,
built on top of a versatile system for data logistics (git-annex) and the most popular distributed
version control system (Git). It adapts principles of open-source software development and
distribution to address the technical challenges of data management, data sharing, and digital
provenance collection across the life cycle of digital objects. DataLad aims to make data
management as easy as managing code. It streamlines procedures to consume, publish, and
update data, for data of any size or type, and to link them as precisely versioned, lightweight
dependencies. DataLad helps to make science more reproducible and FAIR (Wilkinson et al.,
2016). It can capture complete and actionable process provenance of data transformations to
enable automatic re-computation. The DataLad project (datalad.org) delivers a completely
open, pioneering platform for flexible decentralized research data management (RDM) (Hanke,
Pestilli, et al., 2021). It features a Python and a command-line interface, an extensible
architecture, and does not depend on any centralized services but facilitates interoperability
with a plurality of existing tools and services. In order to maximize its utility and target audience, DataLad is available for all major operating systems, and can be integrated into
established workflows and environments with minimal friction.


Statement of Need
Code, data and computing environments are core components of scientific projects. While
the collaborative development and use of research software and code is streamlined with established procedures and infrastructures, such as software distributions, distributed version
control systems, and social coding portals like GitHub, other components of scientific projects
are not as transparently managed or accessible. Data consumption is complicated by disconnected data portals that require a large variety of different data access and authentication
methods. Compared with code in software development, data tend not to be as precisely
identified because data versioning is rarely or only coarsely practiced. Scientific computation
is not reproducible enough, because data provenance, the information of how a digital file
came to be, is often incomplete and rarely automatically captured. Last but not least, in
the absence of standardized data packages, there is no uniform way to declare actionable
data dependencies and derivative relationships between inputs and outputs of a computation. DataLad aims to solve these issues by providing streamlined, transparent management
of code, data, computing environments, and their relationship. It provides targeted interfaces
and interoperability adapters to established scientific and commercial tools and services to
set up unobstructed, unified access to all elements of scientific projects. This unique set of
features enables workflows that are particularly suited for reproducible science, such as actionable process provenance capture for arbitrary command execution that affords automatic
re-execution. To this end, it builds on and extends two established tools for version control
and transport logistics, Git and git-annex.


Why Git and git-annex?
Git is the most popular version control system for software development1
. It is a distributed
content management system, specifically tuned towards managing and collaborating on text
files, and excels at making all committed content reliably and efficiently available to all clones
of a repository. At the same time, Git is not designed to efficiently handle large (e.g., over
a gigabyte) or binary files (see, e.g., Kenlon, 2016). This makes it hard or impossible to
use Git directly for distributed data storage with tailored access to individual files. Gitannex takes advantage of Git’s ability to efficiently manage textual information to overcome
this limitation. File content handled by git-annex is placed into a managed repository annex,
which avoids committing the file content directly to Git. Instead, git-annex commits a compact
reference, typically derived from the checksum of a file’s content, that enables identification
and association of a file name with the content. Using these identifiers, git-annex tracks
content availability across all repository clones and external resources such as URLs pointing
to individual files on the web. Upon user request, git-annex automatically manages data
transport to and from a local repository annex at a granularity of individual files. With this
simple approach, git-annex enables separate and optimized implementations for identification
and transport of arbitrarily large files, using an extensible set of protocols, while retaining the
distributed nature and compatibility with versatile workflows for versioning and collaboration
provided by Git.


What does DataLad add to Git and git-annex?
Easy to use modularization. Research workflows impose additional demands for an efficient research data management platform besides version control and data transport. Many
research datasets contain millions of files, but a large number of files precludes managing
such a dataset in a single Git repository, even if the total storage demand is not huge. Partitioning such datasets into smaller, linked components (e.g., one subdataset per sample in
a dataset comprising thousands) allows for scalable management. Research datasets and
projects can also be heterogeneous, comprising different data sources or evolving data across
different processing stages, and with different pace. Beyond scalability, modularization into
homogeneous components also enables efficient reuse of a selected subset of datasets and for
recording a derivative relationship between datasets. Git’s submodule mechanism provides a
way to nest individual repositories via unambiguously versioned linkage, but Git operations
must still be performed within each individual repository. To achieve modularity without impeding usability, DataLad simplifies working with the resulting hierarchies of Git repositories
via recursive operations across dataset boundaries. With this, DataLad provides a “monorepo”-like user experience in datasets with arbitrarily deep nesting, and makes it trivial to
operate on individual files deep in the hierarchy or entire trees of datasets. A testament of
this is datasets.datalad.org, created as the project’s initial goal to provide a data distribution
with unified access to already available public data archives in neuroscience, such as crcns.org
and openfmri.org. It is curated by the DataLad team and provides, at the time of publication,
streamlined access to over 260 TBs of data across over 5,000 subdatasets from a wide range
of projects and dozens of archives in a fully modularized way.