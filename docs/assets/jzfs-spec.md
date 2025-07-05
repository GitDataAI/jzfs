
# Why JZFS for Data ?

* Chaos has ensued for non-expert end users as data ecosystems progressively develop into complex and siloed systems with a continuous stream of point solutions added to the insane mix. 

* Complex infrastructures requiring consistent maintenance deflect most of the engineering talent from high-value operations, such as developing data applications that directly impact the business and ultimately enhance the ROI of data teams.

* Inflexible and unstable, and therefore, fragile data pipelines constrict data engineering teams as a bottleneck for even simple data operations. It is not uncommon to hear a whole new data pipeline being spawned to answer one specific business question or 1000K data warehouse tables being created from 6K source tables.

Data Consumers suffer from unreliable data quality, Data Producers suffer from duplicated efforts to produce data for ambiguous objectives, and Data Engineers suffer from flooding requests from both data production and consumption sides.

The dearth of exemplary developer experience also robs data developers of the ability to declaratively manage resources, environments, and requests so they can focus completely on data solutions.  

Due to these diversions and the lack of a unified platform, it is nearly impossible for DEs to build short and crisp data-to-insight roadmaps.  

On top of that, it’s a constant struggle to adhere to the organization’s changing data compliance standards as governance and observability become afterthoughts in a maintenance-first setting.  This directly impacts the quality and experience of data that passes through meandering pipelines blotched with miscellaneous integrations.

The concept of having an assembled architecture emerged over time to solve these common problems that infested the data community at large. One tool could tend to a particular problem, and assembling a collection of such tools would solve several issues. But, targeting patches of the problem led to a disconnected basket of solutions ending up with fragile data pipelines and dumping all data to a central lake that eventually created unmanageable data swamps across industries. This augmented the problem by adding the cognitive load of a plethora of tooling that had to be integrated and managed separately through expensive resources and experts.

Data swamps are no better than physical files in the basement-clogged with rich, useful, yet dormant data that businesses are unable to operationalise due to disparate and untrustworthy semantics. Semantic untrustworthiness stems from a chaotic clutter of MDS, overwhelmed with tools, integrations, and unstable pipelines. Another level of semantics is required to understand the low-level semantics, complicating the problem further.

Two distinct features become more apparent with this kind of tooling overwhelm:

1. Progressive overlap in Assembled Systems
   As more tools pop in, they increasingly develop the need to become independently operable, often based on user feedback. For instance, two different point tools, say one for cataloguing and another for governance, are plugged into your data stacks. This incites the need not just to learn the tools’ different philosophies, integrate, and maintain each one from scratch but eventually pop up completely parallel tracks. The governance tool starts requiring a native catalog, and the cataloguing tool requires policies manageable within its system. Now consider the same problem at scale, beyond just two point solutions. Even if we consider the cost of these parallel tracks as secondary, it is essentially a significantly disruptive design flaw that keeps splitting the topology of one unique capability into unmanageable duplicates.

2. Consistent and increasing desire to Decentralise
   What follows from assembled systems is the sudden overwhelm of managing multiple limbs of the system, and therefore, increasing complexity and friction for end users to get their hands on the data. While business domains, such as marketing, sales, support, etc., have to jump multiple hops to achieve the data they need, the organisation feels the pressure to lift all dependencies clogging the central data team and distributing the workload across these domains. Ergo, it was not a surprise to see how the early Data Mesh laid urgent focus on domain ownership, or decentralisation in other words. While the idea seems very appealing on theoretical grounds, how feasible is it in the field? If we lay this idea on any working business model, there are a few consequences:

* Not enough skilled professionals to allocate to each individual domain - Practically, how feasible is the idea of having data teams for each domain?
* Not enough professionals or budget to disrupt existing processes, detangle pipelines, and embed brand-new infrastructures.
* Not enough experts to help train and onboard during migration.

It’s both a skill- and resource-deficit issue. Moreover, with decades spent on evolving data stacks with not much value to show, organisations are not ideally inclined to pour in more investments and efforts to rip and replace their work. In essence, Autonomy instead should become the higher priority over Decentralisation if that is the ultimate objective.

Why - Data Developer Platform
https://datadeveloperplatform.org/why_ddp_for_data/#why-build-a-ddp-for-data-products
