
const neo4j = require('neo4j-driver')
const fs = require("fs");
const { parse } = require("csv-parse");

const useDriver = () => {
    return neo4j.driver(
        'neo4j://159.65.141.247:7474',
        neo4j.auth.basic('neo4j', 'Eu4r1@n')
    )
}

const useGraphQuery = async (driver, query) => {
    const session = driver.session()
    const output = await session.executeRead(async (tx) => {
        const graphTransaction = await tx.run(query)
        console.log(graphTransaction)
    })
    await session.close()
    console.log("finished "+ query)
}

const juris_to_juris_query = (a, b) => {
    return `
    match(juris:Juris), (citation:Juris)
    where juris.unique_id='`+a+`' and citation.unique_id='`+b+`'
    merge (juris) -[:MENTIONS]-> (citation)
    `
}
let neo4jDriver
try{
    neo4jDriver = useDriver()

    neo4jDriver.close()
}catch(error){
    console.log(error)

}
let counter = 0
fs.createReadStream("./juris_to_juris.csv")
    .pipe(parse({ delimiter: ",", from_line: 2 }))
    .on("data",  async(row) =>{
        let query = juris_to_juris_query(row[0],row[1])
        useGraphQuery(neo4jDriver, query);
        if (counter%500 == 0){
            console.log(counter)
        }
        counter+=1
    })

