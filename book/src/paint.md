# 🎨 Paint

The Section `Paint` deals with making the first step.

In classical crypto fashion, we start with the Smart Contracts.

An other option would to use Figma and paint a rectacle.  
An other option would to use Javascript and paint a button.  
An other option would to use NodeJS paint one get request.  
An other option would to use Blender and paint your first vertice.  
An other option would to use Krita and paint your first stroke.  
An other option would to use Excel and paint your first column.  
An other ... I could go on hours with. Anyways lets start.


## Build

1. Setup up ink
2. Initialise a contract `cargo contract new wearo`  
   2.1. (optional) first commit  
   `git init && git branch -m master main && git add . && git commit -m ":zap: (contracts) here we go"`
3. Delete the Boilerplate
4. Add your first variable
5. Add your first function

## Compile

6. Build your little flower `cargo contract build`
7. Fix bugs, if it doesn't want to get shredded into binary.
8. And again, `cargo contract build`, great.

## Call

7. Spin up a node  
   `substrate-contracts-node --dev`
8. Deploy your contract  
   `cargo contract instantiate --suri //Alice --execute`
9. Call your getter  
   `cargo contract call --contract <address> --call get_digital_asset --suri //Alice`
10. Call your setter  
    `cargo contract call --contract <address> --call change_digital_assets --message "ipfs.io/ipfs/0fwoooGREAToooIPFSoooHASHooorouip" --suri //Alice --execute`
11. Call your getter  
    `cargo contract call --contract <address> --call get_digital_asset --suri //Alice`
12. DONE  
    12.1 (optional) commit it  
    `git add . && git commit -m ":rocket: (contracts) gotcha!"`

**Nice. That's it.**

For the people who love videos. Here a quick rundown. 

<iframe width="560" height="315" src="https://www.youtube.com/embed/M0k_jhmFYoo?si=TV5sD-6kNVULx8vI" title="YouTube video player" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" allowfullscreen></iframe>


We covered the following:

- How to think about of generating Ideas
- How to sketch an Idea into an Shape
- How to paint your first step.

I will conclude our doings in the next section.
Its less informative, more lyrical. Its writing for the sake of writing itself. Feel free to call it a day and thanks a lot for going through my little book. I appreciate it a lot.

Next up. Conclusion.
