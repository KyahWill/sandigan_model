
var firebase = require('firebase/app')
var storage = require('firebase/storage')
var fs = require('fs');

const config = {
    apiKey: "AIzaSyD20tTWEAVPSR9Qg5qrxuKmL1_S2WlC89s",
    authDomain: "sandigan-production.firebaseapp.com",
    projectId: "sandigan-production",
    storageBucket: "sandigan-production.appspot.com",
    messagingSenderId: "409594639089",
    appId: "1:409594639089:web:b67be392fb166cfa09ef50"
  
}
var app = firebase.initializeApp(config);
const juris_storage = storage.getStorage(app, 'gs://sandigan-production.appspot.com');
var files = fs.readdirSync('./temp_target/');

const uploadToFirebase = async(files) => {

        // const juris_ref = storage.ref(juris_storage, '_html/'+files[1])
        // let file_value = fs.readFileSync('./temp_target/'+files[1], 'utf8', (err, data) => {
        //     if (err){
        //         console.log(err)
        //     }else{
        //         return data
        //     }
        // })
        // try {
        //     await storage.uploadString(juris_ref,file_value, 'raw',{
        //         contentType: 'text/html',
        //       })
        // } catch (error) {
        //     console.log(error)
        // }
    let log_number = 0
    for (const file of files) {
    
        const juris_ref = storage.ref(juris_storage, 'juris_html/'+file)
        let file_value = fs.readFileSync('./temp_target/'+file, 'utf8', (err, data) => {
            if (err){
                console.log(err)
            }else{
                return data
            }
        })
        try {
            await storage.uploadString(juris_ref,file_value,'raw',{contentType:'text/html'})
        } catch (error) {
            console.log(error)
        }
                console.log("Log # "+ log_number)
        log_number +=1
    }
};
uploadToFirebase(files)  
// Promise.all(files.map((file) => {
//     if (log_number % 500 == 0) {
//         console.log(log_number)
//     }

//     // console.log(juris_ref.fullPath)
//       console.log("result #" +log_number+":" + result)
//     log_number +=1
// }));



// const upload = () => {
//     const juris_ref = storage.ref(juris_storage, 'juris_html/'+file);
//     let filename = './temp_target/'+file
//     let file_value = fs.readFileSync(filename, 'utf8', (err, data) => {
//         if (err){
//             console.log(err)
//         }else{
//             return data
//         }
//     })
//     // console.log(juris_ref.fullPath)
//     storage.uploadString(juris_ref,file_value)
//     console.log("result # index:" + result)
// } 
// let filename = './temp_target/'+files[1]
// let file_value = fs.readFileSync(filename, 'utf8', (err, data) => {
//     if (err){

//         console.log(err)
//     }else{
//         return data
//     }
// })
// const juris_ref = storage.ref(juris_storage, 'juris_html/'+files[1]);
// storage.uploadString(juris_ref,file_value)

// let juris_ref = storage.ref(juris_storage,'juris_html/')
// storage.listAll(juris_ref).then( (results) => {
//     console.log(results)
// })

// console.log(juris_storage.setMaxUploadRetryTime(12000000));

// console.log()
// let log = 1


// files.reduce(async(a, file) =>{
//     // await console.log(file)
//     const juris_ref = await storage.ref(juris_storage, 'juris_html/'+file);
//     const result =await storage.getDownloadURL(juris_ref)
//     console.log("result of "+log+": "+ result  )
//     log = log + 1
//   }, Promise.resolve())
// files.forEach( (file, index ) => {
//     console.log(index + ": "+ file)
//     const juris_ref = storage.ref(juris_storage, 'juris_html/'+file);
//     let filename = './temp_target/'+file
//     let file_value = fs.readFileSync(filename, 'utf8', (err, data) => {
//         if (err){
//             console.log(err)
//         }else{
//             return data
//         }
//     })
//     // console.log(juris_ref.fullPath)
//     storage.uploadString(juris_ref,file_value)
//     console.log("result # index:" + result)
//     if (index%500 == 0) {
//         console.log(index)
//     }
// })



// uploadBytes(storageRef, file).then((snapshot) => {
//   console.log('Uploaded a blob or file!');
// })
